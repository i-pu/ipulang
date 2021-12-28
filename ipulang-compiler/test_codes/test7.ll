; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %"1" = alloca i32, align 4
  store i32 72, i32* %"1", align 4
  %"2" = load i32, i32* %"1", align 4
  %"3" = call i32 @putchar(i32 %"2")
  %"4" = alloca i32, align 4
  store i32 %"3", i32* %"4", align 4
  %"5" = alloca i32, align 4
  store i32 69, i32* %"5", align 4
  %"6" = load i32, i32* %"5", align 4
  %"7" = call i32 @putchar(i32 %"6")
  %"8" = alloca i32, align 4
  store i32 %"7", i32* %"8", align 4
  %"9" = alloca i32, align 4
  store i32 76, i32* %"9", align 4
  %"10" = load i32, i32* %"9", align 4
  %"11" = call i32 @putchar(i32 %"10")
  %"12" = alloca i32, align 4
  store i32 %"11", i32* %"12", align 4
  %"13" = alloca i32, align 4
  store i32 76, i32* %"13", align 4
  %"14" = load i32, i32* %"13", align 4
  %"15" = call i32 @putchar(i32 %"14")
  %"16" = alloca i32, align 4
  store i32 %"15", i32* %"16", align 4
  %"17" = alloca i32, align 4
  store i32 79, i32* %"17", align 4
  %"18" = load i32, i32* %"17", align 4
  %"19" = call i32 @putchar(i32 %"18")
  %"20" = alloca i32, align 4
  store i32 %"19", i32* %"20", align 4
  %"21" = alloca i32, align 4
  store i32 33, i32* %"21", align 4
  %"22" = load i32, i32* %"21", align 4
  %"23" = call i32 @putchar(i32 %"22")
  %"24" = alloca i32, align 4
  store i32 %"23", i32* %"24", align 4
  %"25" = alloca i32, align 4
  store i32 0, i32* %"25", align 4
  %"26" = load i32, i32* %"25", align 4
  ret i32 %"26"
  ret i32 0
}
