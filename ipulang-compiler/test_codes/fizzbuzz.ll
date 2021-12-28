; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @fizz() {
entry:
  %_v1 = alloca i32, align 4
  store i32 102, i32* %_v1, align 4
  %_v2 = load i32, i32* %_v1, align 4
  %_v3 = call i32 @putchar(i32 %_v2)
  %_v4 = alloca i32, align 4
  store i32 %_v3, i32* %_v4, align 4
  %_v5 = alloca i32, align 4
  store i32 105, i32* %_v5, align 4
  %_v6 = load i32, i32* %_v5, align 4
  %_v7 = call i32 @putchar(i32 %_v6)
  %_v8 = alloca i32, align 4
  store i32 %_v7, i32* %_v8, align 4
  %_v9 = alloca i32, align 4
  store i32 122, i32* %_v9, align 4
  %_v10 = load i32, i32* %_v9, align 4
  %_v11 = call i32 @putchar(i32 %_v10)
  %_v12 = alloca i32, align 4
  store i32 %_v11, i32* %_v12, align 4
  %_v13 = alloca i32, align 4
  store i32 122, i32* %_v13, align 4
  %_v14 = load i32, i32* %_v13, align 4
  %_v15 = call i32 @putchar(i32 %_v14)
  %_v16 = alloca i32, align 4
  store i32 %_v15, i32* %_v16, align 4
  ret i32 0
}

define i32 @buzz() {
entry:
  %_v17 = alloca i32, align 4
  store i32 98, i32* %_v17, align 4
  %_v18 = load i32, i32* %_v17, align 4
  %_v19 = call i32 @putchar(i32 %_v18)
  %_v20 = alloca i32, align 4
  store i32 %_v19, i32* %_v20, align 4
  %_v21 = alloca i32, align 4
  store i32 117, i32* %_v21, align 4
  %_v22 = load i32, i32* %_v21, align 4
  %_v23 = call i32 @putchar(i32 %_v22)
  %_v24 = alloca i32, align 4
  store i32 %_v23, i32* %_v24, align 4
  %_v25 = alloca i32, align 4
  store i32 122, i32* %_v25, align 4
  %_v26 = load i32, i32* %_v25, align 4
  %_v27 = call i32 @putchar(i32 %_v26)
  %_v28 = alloca i32, align 4
  store i32 %_v27, i32* %_v28, align 4
  %_v29 = alloca i32, align 4
  store i32 122, i32* %_v29, align 4
  %_v30 = load i32, i32* %_v29, align 4
  %_v31 = call i32 @putchar(i32 %_v30)
  %_v32 = alloca i32, align 4
  store i32 %_v31, i32* %_v32, align 4
  ret i32 0
}

define i32 @judge(i32 %0) {
entry:
  %i = alloca i32, align 4
  store i32 %0, i32* %i, align 4
  %_v33 = alloca i32, align 4
  store i32 3, i32* %_v33, align 4
  %_v34 = load i32, i32* %i, align 4
  %_v35 = load i32, i32* %_v33, align 4
  %_v36 = srem i32 %_v34, %_v35
  %_v37 = alloca i32, align 4
  store i32 %_v36, i32* %_v37, align 4
  %_v38 = alloca i32, align 4
  store i32 0, i32* %_v38, align 4
  %_v39 = load i32, i32* %_v37, align 4
  %_v40 = load i32, i32* %_v38, align 4
  %_v41 = icmp eq i32 %_v39, %_v40
  %_v42 = alloca i1, align 1
  store i1 %_v41, i1* %_v42, align 1
  %_v43 = load i1, i1* %_v42, align 1
  %_v44 = icmp ne i1 %_v43, false
  br i1 %_v44, label %label45, label %label46

label45:                                          ; preds = %entry
  %_v48 = call i32 @fizz()
  %_v49 = alloca i32, align 4
  store i32 %_v48, i32* %_v49, align 4
  %_v50 = alloca i32, align 4
  store i32 10, i32* %_v50, align 4
  %_v51 = load i32, i32* %_v50, align 4
  %_v52 = call i32 @putchar(i32 %_v51)
  %_v53 = alloca i32, align 4
  store i32 %_v52, i32* %_v53, align 4
  br label %label47

label46:                                          ; preds = %entry
  %_v54 = alloca i32, align 4
  store i32 5, i32* %_v54, align 4
  %_v55 = load i32, i32* %i, align 4
  %_v56 = load i32, i32* %_v54, align 4
  %_v57 = srem i32 %_v55, %_v56
  %_v58 = alloca i32, align 4
  store i32 %_v57, i32* %_v58, align 4
  %_v59 = alloca i32, align 4
  store i32 0, i32* %_v59, align 4
  %_v60 = load i32, i32* %_v58, align 4
  %_v61 = load i32, i32* %_v59, align 4
  %_v62 = icmp eq i32 %_v60, %_v61
  %_v63 = alloca i1, align 1
  store i1 %_v62, i1* %_v63, align 1
  %_v64 = load i1, i1* %_v63, align 1
  %_v65 = icmp ne i1 %_v64, false
  br i1 %_v65, label %label66, label %label67

label47:                                          ; preds = %label68, %label45
  ret i32 0

label66:                                          ; preds = %label46
  %_v69 = call i32 @buzz()
  %_v70 = alloca i32, align 4
  store i32 %_v69, i32* %_v70, align 4
  %_v71 = alloca i32, align 4
  store i32 10, i32* %_v71, align 4
  %_v72 = load i32, i32* %_v71, align 4
  %_v73 = call i32 @putchar(i32 %_v72)
  %_v74 = alloca i32, align 4
  store i32 %_v73, i32* %_v74, align 4
  br label %label68

label67:                                          ; preds = %label46
  %_v75 = alloca i32, align 4
  store i32 15, i32* %_v75, align 4
  %_v76 = load i32, i32* %i, align 4
  %_v77 = load i32, i32* %_v75, align 4
  %_v78 = srem i32 %_v76, %_v77
  %_v79 = alloca i32, align 4
  store i32 %_v78, i32* %_v79, align 4
  %_v80 = alloca i32, align 4
  store i32 0, i32* %_v80, align 4
  %_v81 = load i32, i32* %_v79, align 4
  %_v82 = load i32, i32* %_v80, align 4
  %_v83 = icmp eq i32 %_v81, %_v82
  %_v84 = alloca i1, align 1
  store i1 %_v83, i1* %_v84, align 1
  %_v85 = load i1, i1* %_v84, align 1
  %_v86 = icmp ne i1 %_v85, false
  br i1 %_v86, label %label87, label %label88

label68:                                          ; preds = %label89, %label66
  br label %label47

label87:                                          ; preds = %label67
  %_v90 = call i32 @fizz()
  %_v91 = alloca i32, align 4
  store i32 %_v90, i32* %_v91, align 4
  %_v92 = call i32 @buzz()
  %_v93 = alloca i32, align 4
  store i32 %_v92, i32* %_v93, align 4
  %_v94 = alloca i32, align 4
  store i32 10, i32* %_v94, align 4
  %_v95 = load i32, i32* %_v94, align 4
  %_v96 = call i32 @putchar(i32 %_v95)
  %_v97 = alloca i32, align 4
  store i32 %_v96, i32* %_v97, align 4
  br label %label89

label88:                                          ; preds = %label67
  %_v98 = alloca i32, align 4
  store i32 48, i32* %_v98, align 4
  %_v99 = load i32, i32* %i, align 4
  %_v100 = load i32, i32* %_v98, align 4
  %_v101 = add i32 %_v99, %_v100
  %_v102 = alloca i32, align 4
  store i32 %_v101, i32* %_v102, align 4
  %_v103 = load i32, i32* %_v102, align 4
  %_v104 = call i32 @putchar(i32 %_v103)
  %_v105 = alloca i32, align 4
  store i32 %_v104, i32* %_v105, align 4
  %_v106 = alloca i32, align 4
  store i32 10, i32* %_v106, align 4
  %_v107 = load i32, i32* %_v106, align 4
  %_v108 = call i32 @putchar(i32 %_v107)
  %_v109 = alloca i32, align 4
  store i32 %_v108, i32* %_v109, align 4
  br label %label89

label89:                                          ; preds = %label88, %label87
  br label %label68
}

define i32 @main() {
entry:
  %i = alloca i32, align 4
  store i32 0, i32* %i, align 4
  br label %label110

label110:                                         ; preds = %label112, %entry
  %_v114 = alloca i32, align 4
  store i32 30, i32* %_v114, align 4
  %_v115 = load i32, i32* %i, align 4
  %_v116 = load i32, i32* %_v114, align 4
  %_v117 = icmp slt i32 %_v115, %_v116
  %_v118 = alloca i1, align 1
  store i1 %_v117, i1* %_v118, align 1
  %_v119 = load i1, i1* %_v118, align 1
  %_v120 = icmp ne i1 %_v119, false
  br i1 %_v120, label %label111, label %label113

label111:                                         ; preds = %label110
  %_v121 = load i32, i32* %i, align 4
  %_v122 = call i32 @judge(i32 %_v121)
  %_v123 = alloca i32, align 4
  store i32 %_v122, i32* %_v123, align 4
  br label %label112

label112:                                         ; preds = %label111
  %_v124 = alloca i32, align 4
  store i32 1, i32* %_v124, align 4
  %_v125 = load i32, i32* %i, align 4
  %_v126 = load i32, i32* %_v124, align 4
  %_v127 = add i32 %_v125, %_v126
  %_v128 = alloca i32, align 4
  store i32 %_v127, i32* %_v128, align 4
  %_v129 = load i32, i32* %_v128, align 4
  store i32 %_v129, i32* %i, align 4
  br label %label110

label113:                                         ; preds = %label110
  ret i32 0
}
