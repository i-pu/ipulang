; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @fib(i32 %0) {
entry:
  %i = alloca i32, align 4
  store i32 %0, i32* %i, align 4
  %_v1 = alloca i32, align 4
  store i32 1, i32* %_v1, align 4
  %_v2 = load i32, i32* %i, align 4
  %_v3 = load i32, i32* %_v1, align 4
  %_v4 = icmp sle i32 %_v2, %_v3
  %_v5 = alloca i1, align 1
  store i1 %_v4, i1* %_v5, align 1
  %_v6 = load i1, i1* %_v5, align 1
  %_v7 = icmp ne i1 %_v6, false
  br i1 %_v7, label %label8, label %label9

label8:                                           ; preds = %entry
  %_v11 = alloca i32, align 4
  store i32 1, i32* %_v11, align 4
  %_v12 = load i32, i32* %_v11, align 4
  ret i32 %_v12
  br label %label10

label9:                                           ; preds = %entry
  %_v13 = alloca i32, align 4
  store i32 1, i32* %_v13, align 4
  %_v14 = load i32, i32* %i, align 4
  %_v15 = load i32, i32* %_v13, align 4
  %_v16 = sub i32 %_v14, %_v15
  %_v17 = alloca i32, align 4
  store i32 %_v16, i32* %_v17, align 4
  %_v18 = load i32, i32* %_v17, align 4
  %_v19 = call i32 @fib(i32 %_v18)
  %_v20 = alloca i32, align 4
  store i32 %_v19, i32* %_v20, align 4
  %_v21 = alloca i32, align 4
  store i32 2, i32* %_v21, align 4
  %_v22 = load i32, i32* %i, align 4
  %_v23 = load i32, i32* %_v21, align 4
  %_v24 = sub i32 %_v22, %_v23
  %_v25 = alloca i32, align 4
  store i32 %_v24, i32* %_v25, align 4
  %_v26 = load i32, i32* %_v25, align 4
  %_v27 = call i32 @fib(i32 %_v26)
  %_v28 = alloca i32, align 4
  store i32 %_v27, i32* %_v28, align 4
  %_v29 = load i32, i32* %_v20, align 4
  %_v30 = load i32, i32* %_v28, align 4
  %_v31 = add i32 %_v29, %_v30
  %_v32 = alloca i32, align 4
  store i32 %_v31, i32* %_v32, align 4
  %_v33 = load i32, i32* %_v32, align 4
  ret i32 %_v33
  br label %label10

label10:                                          ; preds = %label9, %label8
  ret i32 0
}

define i32 @main() {
entry:
  %_v34 = alloca i32, align 4
  store i32 42, i32* %_v34, align 4
  %_v35 = load i32, i32* %_v34, align 4
  %_v36 = call i32 @fib(i32 %_v35)
  %_v37 = alloca i32, align 4
  store i32 %_v36, i32* %_v37, align 4
  %_v38 = load i32, i32* %_v37, align 4
  ret i32 %_v38
  ret i32 0
}
