; ModuleID = 'main'
source_filename = "main"

define i32 @main(i32 %0) {
entry:
  %a = alloca i32, align 4
  %b = alloca i32, align 4
  store i32 3, i32* %a, align 4
  store i32 5, i32* %b, align 4
  %bload = load i32, i32* %b, align 4
  %aload = load i32, i32* %a, align 4
  %c = add i32 %aload, %bload
  %d = add i32 %c, %0
  ret i32 %d
}
