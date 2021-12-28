; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @f(i32 %0, i32 %1) {
entry:
  %a = alloca i32, align 4
  store i32 %0, i32* %a, align 4
  %b = alloca i32, align 4
  store i32 %1, i32* %b, align 4


label1:                                           ; No predecessors!
  %_v2 = load i32, i32* %a, align 4
  %_v3 = load i32, i32* %b, align 4
  %_v4 = add i32 %_v2, %_v3
  %_v5 = alloca i32, align 4
  store i32 %_v4, i32* %_v5, align 4
  %_v6 = load i32, i32* %_v5, align 4
  ret i32 %_v6
  ret i32 0
}

define i32 @main() {
entry:

label7:                                           ; No predecessors!
  %_v8 = alloca i32, align 4
  store i32 1, i32* %_v8, align 4
  %_v9 = load i32, i32* %_v8, align 4
  %_v10 = alloca i32, align 4
  store i32 0, i32* %_v10, align 4
  %_v11 = load i32, i32* %_v10, align 4
  %_v12 = call i32 @f(i32 %_v9, i32 %_v11)
  %_v13 = alloca i32, align 4
  store i32 %_v12, i32* %_v13, align 4
  %_v14 = alloca i32, align 4
  store i32 3, i32* %_v14, align 4
  %_v15 = load i32, i32* %_v14, align 4
  %_v16 = alloca i32, align 4
  store i32 9, i32* %_v16, align 4
  %_v17 = load i32, i32* %_v16, align 4
  %_v18 = call i32 @f(i32 %_v15, i32 %_v17)
  %_v19 = alloca i32, align 4
  store i32 %_v18, i32* %_v19, align 4
  %_v20 = load i32, i32* %_v13, align 4
  %_v21 = load i32, i32* %_v19, align 4
  %_v22 = add i32 %_v20, %_v21
  %_v23 = alloca i32, align 4
  store i32 %_v22, i32* %_v23, align 4
  %_v24 = load i32, i32* %_v23, align 4
  ret i32 %_v24
  ret i32 0
}
