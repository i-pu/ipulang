; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @f(i32 %0) {
entry:
  %a = alloca i32, align 4
  store i32 %0, i32* %a, align 4
  %"1" = alloca i32, align 4
  store i32 1, i32* %"1", align 4
  %"2" = load i32, i32* %a, align 4
  %"3" = load i32, i32* %"1", align 4
  %"4" = add i32 %"2", %"3"
  %"5" = alloca i32, align 4
  store i32 %"4", i32* %"5", align 4
  %"6" = load i32, i32* %"5", align 4
  ret i32 %"6"
  ret i32 0
}

define i32 @main() {
entry:
  %"7" = alloca i32, align 4
  store i32 1, i32* %"7", align 4
  %"8" = load i32, i32* %"7", align 4
  %"9" = call i32 @f(i32 %"8")
  %"10" = alloca i32, align 4
  store i32 %"9", i32* %"10", align 4
  %"11" = load i32, i32* %"10", align 4
  ret i32 %"11"
  ret i32 0
}
