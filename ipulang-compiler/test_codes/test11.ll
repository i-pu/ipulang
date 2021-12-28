; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %_v1 = alloca i32, align 4
  store i32 1, i32* %_v1, align 4
  %_v2 = load i32, i32* %_v1, align 4
  %a = alloca i32, align 4
  store i32 %_v2, i32* %a, align 4
  %_v7 = alloca i32, align 4
  store i32 10, i32* %_v7, align 4
  %_v8 = load i32, i32* %_v7, align 4
  %i = alloca i32, align 4
  store i32 %_v8, i32* %i, align 4
  br label %label3

label3:                                           ; preds = %label5, %entry
  %_v9 = alloca i32, align 4
  store i32 0, i32* %_v9, align 4
  %_v10 = load i32, i32* %_v9, align 4
  %_v11 = icmp ne i32 %_v10, 0
  br i1 %_v11, label %label4, label %label6

label4:                                           ; preds = %label3
  %_v12 = load i32, i32* %a, align 4
  %_v13 = load i32, i32* %i, align 4
  %_v14 = add i32 %_v12, %_v13
  %_v15 = alloca i32, align 4
  store i32 %_v14, i32* %_v15, align 4
  %_v16 = load i32, i32* %_v15, align 4
  store i32 %_v16, i32* %a, align 4
  br label %label5

label5:                                           ; preds = %label4
  %_v17 = alloca i32, align 4
  store i32 1, i32* %_v17, align 4
  %_v18 = load i32, i32* %i, align 4
  %_v19 = load i32, i32* %_v17, align 4
  %_v20 = add i32 %_v18, %_v19
  %_v21 = alloca i32, align 4
  store i32 %_v20, i32* %_v21, align 4
  %_v22 = load i32, i32* %_v21, align 4
  store i32 %_v22, i32* %i, align 4
  br label %label3

label6:                                           ; preds = %label3
  %_v23 = load i32, i32* %a, align 4
  ret i32 %_v23
  ret i32 0
}
