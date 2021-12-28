; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %_v1 = alloca i32, align 4
  store i32 1, i32* %_v1, align 4
  %_v2 = alloca i32, align 4
  store i32 2, i32* %_v2, align 4
  %_v3 = load i32, i32* %_v1, align 4
  %_v4 = load i32, i32* %_v2, align 4
  %_v5 = icmp slt i32 %_v3, %_v4
  %_v6 = alloca i1, align 1
  store i1 %_v5, i1* %_v6, align 1
  ret i32 0
}
