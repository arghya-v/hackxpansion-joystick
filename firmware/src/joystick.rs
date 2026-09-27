use alloc::boxed::Box;

use core::{
    future::Future,
    pin::Pin,
};

use embassy_rp::{
    Peri,
    adc::AdcPin,
    gpio::AnyPin,
};

use xpanse_api::interfaces::{
    adc::{self, AdcError},
    buttons::{A, Button, pin_button},
};

trait AdcAxis: Send {
    fn read<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<u16, AdcError>> + Send + 'a>>;
}

struct PinAxis<P>
where
    P: AdcPin + Send + 'static,
{
    pin: Peri<'static, P>,
}

impl<P> AdcAxis for PinAxis<P>
where
    P: AdcPin + Send + 'static,
{
    fn read<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<u16, AdcError>> + Send + 'a>> {
        Box::pin(async move {
            adc::read_adc_pin(
                &mut self.pin,
                embassy_rp::gpio::Pull::None,
            )
            .await
        })
    }
}

pub struct Joystick {
    x: Box<dyn AdcAxis>,
    y: Box<dyn AdcAxis>,
    button: Box<dyn Button<A>>,
}

impl Joystick {
    pub fn new<X, Y>(
        x: Peri<'static, X>,
        y: Peri<'static, Y>,
        button: Peri<'static, AnyPin>,
    ) -> Self
    where
        X: AdcPin + Send + 'static,
        Y: AdcPin + Send + 'static,
    {
        Self {
            x: Box::new(PinAxis { pin: x }),
            y: Box::new(PinAxis { pin: y }),
            button: pin_button::<A>(button),
        }
    }

    pub async fn read_x(&mut self) -> Result<u16, AdcError> {
        self.x.read().await
    }

    pub async fn read_y(&mut self) -> Result<u16, AdcError> {
        self.y.read().await
    }

    pub async fn read(&mut self) -> Result<(u16, u16), AdcError> {
        let x = self.read_x().await?;
        let y = self.read_y().await?;
        Ok((x, y))
    }

    pub fn is_pressed(&self) -> bool {
        self.button.is_pressed()
    }

    pub async fn wait_for_pressed(&mut self) {
        self.button.wait_for_pressed().await;
    }
}