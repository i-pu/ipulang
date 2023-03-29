use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use inkwell;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::*;
use inkwell::values::*;
use ipulang_parser::types::Type;

/// コード生成時のための情報
pub struct Env<'ll> {
    pub module: Module<'ll>,
    pub ctx: &'ll Context,
    /// function id -> variable id -> PointerValue
    /// 宣言されている変数一覧
    pub variables: HashMap<String, HashMap<String, PointerValue<'ll>>>,
    /// compilerが作った一時変数の個数
    pub var_count: Rc<Cell<usize>>,
    /// 宣言されている関数一覧
    // TODO: 後から宣言できるようにする
    pub functions: HashMap<String, FunctionValue<'ll>>,

    /// 現在の builder
    pub builder: Builder<'ll>,

    /// 現在のfunction id
    pub function: String,
    /// 現在の FunctionValue
    pub function_value: Option<FunctionValue<'ll>>,
}

impl<'ll> Env<'ll> {
    pub fn new(ctx: &'ll Context) -> Self {
        // module
        let module = ctx.create_module("main");

        let mut functions = HashMap::new();

        // decrare putchar(i32): i32
        let i32_type = ctx.i32_type();
        let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
        let putchar_val = module.add_function("putchar", putchar_type, None);
        functions.insert("putchar".to_owned(), putchar_val);

        // decrate getchar(): i32
        let getchar_type = i32_type.fn_type(&[], false);
        let getchar_val = module.add_function("getchar", getchar_type, None);
        functions.insert("getchar".to_owned(), getchar_val);

        Self {
            ctx: ctx,
            module: module,
            variables: HashMap::new(),
            var_count: Rc::new(Cell::new(0)),
            functions: functions,
            builder: ctx.create_builder(),
            function: "".to_owned(),
            function_value: None,
        }
    }

    pub fn get_tmp_var_id(&self) -> String {
        let tmp = self.var_count.clone();
        (*tmp).set(tmp.get() + 1);
        format!("_v{}", self.var_count.get().to_string())
    }

    pub fn get_tmp_label_id(&self) -> String {
        let tmp = self.var_count.clone();
        (*tmp).set(tmp.get() + 1);
        format!("label{}", self.var_count.get().to_string())
    }

    /// 関数に変数があるかどうか
    pub fn contains(&self, name: String) -> bool {
        self.variables
            .get(&self.function)
            .map(|m: &HashMap<String, PointerValue>| m.contains_key(&name))
            .unwrap_or(false)
    }

    /// 関数に宣言されている変数を取得
    pub fn get_variable(&self, name: String) -> Option<&'_ PointerValue<'ll>> {
        self.variables
            .get(&self.function)
            .map(|m| m.get(&name))
            .flatten()
    }

    /// 関数に変数情報を登録する
    pub fn set_variable(&mut self, name: String, value: PointerValue<'ll>) {
        let map = self.variables.get_mut(&self.function).unwrap();
        map.insert(name, value);
    }

    /// PointerValueをIntValueに変換する
    /// IntValueの名前は任意
    pub fn point_to_int(&self, ptr: PointerValue<'ll>, int_id: Option<String>) -> IntValue {
        let var_id = self.get_tmp_var_id();
        let tmp = self.builder.build_load(ptr, &var_id);
        tmp.into_int_value()
    }

    /// IntValueをPointerValueに変換する
    /// IntValueの名前は任意
    pub fn int_to_point(&self, int: IntValue<'ll>, ptr_id: Option<&str>) -> PointerValue<'ll> {
        if let Some(ptr_id) = ptr_id {
            let ptr: PointerValue = self.builder.build_alloca(int.get_type(), ptr_id);
            self.builder.build_store(ptr, int);
            ptr
        } else {
            let var_id = self.get_tmp_var_id();
            let ptr: PointerValue = self.builder.build_alloca(int.get_type(), &var_id);
            self.builder.build_store(ptr, int);
            ptr
        }
    }

    pub fn get_llvm_fn_type(&self, typ: Type) -> Option<BasicTypeEnum<'ll>> {
        match typ {
            Type::Unit => None,
            Type::Int32 => Some(self.ctx.i32_type().into()),
            Type::Int64 => Some(self.ctx.i64_type().into()),
            Type::Bool => Some(self.ctx.bool_type().into()),
            _ => panic!("type: {} is unknown", typ),
        }
    }

    pub fn get_llvm_value_type(&self, typ: Type) -> BasicMetadataTypeEnum<'ll> {
        match typ {
            Type::Int32 => self.ctx.i32_type().into(),
            Type::Int64 => self.ctx.i64_type().into(),
            Type::Bool => self.ctx.bool_type().into(),
            _ => panic!("type: {} is unknown", typ),
        }
    }
}
