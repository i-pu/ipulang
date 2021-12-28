; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %_v1 = alloca i32, align 4
  store i32 0, i32* %_v1, align 4
  %_v2 = load i32, i32* %_v1, align 4
  %_v3 = icmp ne i32 %_v2, 0
  br i1 %_v3, label %label4, label %label5

label4:                                           ; preds = %entry
  %_v7 = alloca i32, align 4
  store i32 1, i32* %_v7, align 4
  %_v8 = alloca i32, align 4
  store i32 2, i32* %_v8, align 4
  %_v9 = load i32, i32* %_v7, align 4
  %_v10 = load i32, i32* %_v8, align 4
  %_v11 = mul i32 %_v9, %_v10
  %_v12 = alloca i32, align 4
  store i32 %_v11, i32* %_v12, align 4
  %_v13 = load i32, i32* %_v12, align 4
  ret i32 %_v13
  br label %label6

label5:                                           ; preds = %entry
  %_v14 = alloca i32, align 4
  store i32 0, i32* %_v14, align 4
  %_v15 = load i32, i32* %_v14, align 4
  %_v16 = icmp ne i32 %_v15, 0
  br i1 %_v16, label %label17, label %label18

label6:                                           ; preds = %label19, %label4
  %_v24 = alloca i32, align 4
  store i32 0, i32* %_v24, align 4
  %_v25 = load i32, i32* %_v24, align 4
  ret i32 %_v25
  ret i32 0

label17:                                          ; preds = %label5
  %_v20 = alloca i32, align 4
  store i32 3, i32* %_v20, align 4
  %_v21 = load i32, i32* %_v20, align 4
  ret i32 %_v21
  br label %label19

label18:                                          ; preds = %label5
  %_v22 = alloca i32, align 4
  store i32 4, i32* %_v22, align 4
  %_v23 = load i32, i32* %_v22, align 4
  ret i32 %_v23
  br label %label19

label19:                                          ; preds = %label18, %label17
  br label %label6
}
