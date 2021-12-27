; ModuleID = 'main'
source_filename = "main"

define i32 @a(i32 %0) {
entry:
  %"1" = alloca i32, align 4
  store i32 0, i32* %"1", align 4
  %"2" = load i32, i32* %"1", align 4
  ret i32 %"2"
  ret i32 0
}

define i32 @main(i32 %0) {
entry:
  %"3" = alloca i32, align 4
  store i32 6, i32* %"3", align 4
  %"4" = load i32, i32* %"3", align 4
  %a = alloca i32, align 4
  store i32 %"4", i32* %a, align 4
  %"5" = alloca i32, align 4
  store i32 2, i32* %"5", align 4
  %"6" = load i32, i32* %"5", align 4
  %"7" = load i32, i32* %a, align 4
  %"8" = add i32 %"6", %"7"
  %"9" = alloca i32, align 4
  store i32 %"8", i32* %"9", align 4
  %"10" = alloca i32, align 4
  store i32 4, i32* %"10", align 4
  %"11" = alloca i32, align 4
  store i32 1, i32* %"11", align 4
  %"12" = load i32, i32* %"10", align 4
  %"13" = load i32, i32* %"11", align 4
  %"14" = sdiv i32 %"12", %"13"
  %"15" = alloca i32, align 4
  store i32 %"14", i32* %"15", align 4
  %"16" = load i32, i32* %"9", align 4
  %"17" = load i32, i32* %"15", align 4
  %"18" = mul i32 %"16", %"17"
  %"19" = alloca i32, align 4
  store i32 %"18", i32* %"19", align 4
  %"20" = load i32, i32* %"19", align 4
  %b = alloca i32, align 4
  store i32 %"20", i32* %b, align 4
  %"21" = load i32, i32* %b, align 4
  ret i32 %"21"
  ret i32 0
}
