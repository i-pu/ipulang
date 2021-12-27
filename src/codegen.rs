use anyhow::{Error, Result};
use inkwell;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::values::IntValue;
use inkwell::OptimizationLevel;

pub fn jit_compile() -> Result<(), Box<Error>> {
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

    // let three = i32_type.const_int(3, false);
    // let five = i32_type.const_int(5, false);
    // let add2 = builder.build_int_add(three, five, "add2");
    let f = i32_type.const_int(16, false);

    // let builder2 = context.create_builder();
    // let a = builder2.build_alloca(context.i32_type(), "a");
    // let aa = builder2.build_store(a, three);
    // let aaa = builder2.build_load(a, "a");

    // let res = builder.build_int_add(aaa.into_int_value(), five, "");

    // block
    let basic_block = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(basic_block);

    let param0 = main_fn.get_nth_param(0).unwrap().into_int_value();
    let newparam0 = builder.build_int_add(param0, f, "newparam0");
    // TODO: code gen
    // code_gen(&context, &builder);
    builder.build_return(Some(&newparam0));

    // module.print_to_stderr();
    module.print_to_file("test.ll").unwrap();

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
