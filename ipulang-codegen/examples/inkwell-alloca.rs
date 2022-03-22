use anyhow::{Error, Result};
use inkwell;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::values::IntValue;
use inkwell::OptimizationLevel;

pub fn main() -> Result<(), Box<Error>> {
    let context = Context::create();

    // module
    let module = context.create_module("main");
    // builder
    let builder = context.create_builder();

    // i32
    let i32_type = context.i32_type();

    // main: () -> i32
    let main_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let main_fn = module.add_function("main", main_fn_type, None);

    // block
    let basic_block = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(basic_block);

    // 引数を使う時
    let param0 = main_fn.get_nth_param(0).unwrap().into_int_value();

    // a = 3, b = 5
    let a = builder.build_alloca(context.i32_type(), "a");
    let b = builder.build_alloca(context.i32_type(), "b");
    let three = i32_type.const_int(3, false);
    let five = i32_type.const_int(5, false);
    builder.build_store(a, three);
    builder.build_store(b, five);

    // load a, b
    let bload = builder.build_load(b, "bload");
    let aload = builder.build_load(a, "aload");

    // add a, b
    let c = builder.build_int_add(aload.into_int_value(), bload.into_int_value(), "c");

    // add c, param0
    let d = builder.build_int_add(c, param0, "d");

    // TODO: code gen
    // code_gen(&context, &builder);

    builder.build_return(Some(&d));

    // module.print_to_stderr();
    module.print_to_file("examples/inkwell-alloca.ll").unwrap();

    // Ok(())

    // prepare jit engine & execute
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    unsafe {
        execution_engine
            // .get_function("main")
            .get_function::<unsafe extern "C" fn()>("main")
            .unwrap()
            .call();
    };
    Ok(())
}
