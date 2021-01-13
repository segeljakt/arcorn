pub mod state;

use arcon::prelude::state::Backend;
use arcon::prelude::state::Value;
use arcon::prelude::ArconElement;
use arcon::prelude::ArconNever;
use arcon::prelude::ArconResult;
use arcon::prelude::ComponentDefinition;
use arcon::prelude::Operator;
use arcon::prelude::OperatorContext;
use arcon::prelude::OperatorResult;

pub struct MyOperator {}


impl Operator for MyOperator {
    type IN = i32;
    type OUT = i32;
    type TimerState = ArconNever;
    type OperatorState = ();

    arcon::ignore_timeout!();
    arcon::ignore_persist!();

    fn handle_element(
        &mut self,
        element: ArconElement<Self::IN>,
        ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
    ) -> OperatorResult<()> {
        todo!()
    }
}
