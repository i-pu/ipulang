; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @f(i32 %0, i32 %1) {
entry:
  %a = alloca i32, align 4
  store i32 %0, i32* %a, align 4
  %b = alloca i32, align 4
  store i32 %1, i32* %b, align 4
  %"1" = load i32, i32* %a, align 4
  %"2" = load i32, i32* %b, align 4
  %"3" = add i32 %"1", %"2"
  %"4" = alloca i32, align 4
  store i32 %"3", i32* %"4", align 4
  %"5" = load i32, i32* %"4", align 4
  ret i32 %"5"
  ret i32 0
}

define i32 @main() {
entry:
  %"6" = alloca i32, align 4
  store i32 1, i32* %"6", align 4
  %"7" = load i32, i32* %"6", align 4
  %"8" = alloca i32, align 4
  store i32 0, i32* %"8", align 4
  %"9" = load i32, i32* %"8", align 4
  %"10" = call i32 @f(i32 %"7", i32 %"9")
  %"11" = alloca i32, align 4
  store i32 %"10", i32* %"11", align 4
  %"12" = alloca i32, align 4
  store i32 3, i32* %"12", align 4
  %"13" = load i32, i32* %"12", align 4
  %"14" = alloca i32, align 4
  store i32 9, i32* %"14", align 4
  %"15" = load i32, i32* %"14", align 4
  %"16" = call i32 @f(i32 %"13", i32 %"15")
  %"17" = alloca i32, align 4
  store i32 %"16", i32* %"17", align 4
  %"18" = load i32, i32* %"11", align 4
  %"19" = load i32, i32* %"17", align 4
  %"20" = add i32 %"18", %"19"
  %"21" = alloca i32, align 4
  store i32 %"20", i32* %"21", align 4
  %"22" = load i32, i32* %"21", align 4
  ret i32 %"22"
  ret i32 0
}
