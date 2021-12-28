	.text
	.file	"main"
	.globl	fizz                            # -- Begin function fizz
	.p2align	4, 0x90
	.type	fizz,@function
fizz:                                   # @fizz
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movl	$102, 36(%rsp)
	movl	$102, %edi
	callq	putchar@PLT
	movl	%eax, 32(%rsp)
	movl	$105, 28(%rsp)
	movl	$105, %edi
	callq	putchar@PLT
	movl	%eax, 24(%rsp)
	movl	$122, 20(%rsp)
	movl	$122, %edi
	callq	putchar@PLT
	movl	%eax, 16(%rsp)
	movl	$122, 12(%rsp)
	movl	$122, %edi
	callq	putchar@PLT
	movl	%eax, 8(%rsp)
	xorl	%eax, %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	fizz, .Lfunc_end0-fizz
	.cfi_endproc
                                        # -- End function
	.globl	buzz                            # -- Begin function buzz
	.p2align	4, 0x90
	.type	buzz,@function
buzz:                                   # @buzz
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movl	$98, 36(%rsp)
	movl	$98, %edi
	callq	putchar@PLT
	movl	%eax, 32(%rsp)
	movl	$117, 28(%rsp)
	movl	$117, %edi
	callq	putchar@PLT
	movl	%eax, 24(%rsp)
	movl	$122, 20(%rsp)
	movl	$122, %edi
	callq	putchar@PLT
	movl	%eax, 16(%rsp)
	movl	$122, 12(%rsp)
	movl	$122, %edi
	callq	putchar@PLT
	movl	%eax, 8(%rsp)
	xorl	%eax, %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	buzz, .Lfunc_end1-buzz
	.cfi_endproc
                                        # -- End function
	.globl	judge                           # -- Begin function judge
	.p2align	4, 0x90
	.type	judge,@function
judge:                                  # @judge
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$32, %rsp
	movl	%edi, -4(%rbp)
	movl	$3, -20(%rbp)
	movslq	%edi, %rax
	imulq	$1431655766, %rax, %rcx         # imm = 0x55555556
	movq	%rcx, %rdx
	shrq	$63, %rdx
	shrq	$32, %rcx
	addl	%edx, %ecx
	leal	(%rcx,%rcx,2), %ecx
	subl	%ecx, %eax
	movl	%eax, -16(%rbp)
	movl	$0, -12(%rbp)
	sete	-5(%rbp)
	jne	.LBB2_3
# %bb.1:                                # %label45
	callq	fizz@PLT
	jmp	.LBB2_2
.LBB2_3:                                # %label46
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$5, -16(%rax)
	movslq	-4(%rbp), %rax
	imulq	$1717986919, %rax, %rcx         # imm = 0x66666667
	movq	%rcx, %rdx
	shrq	$63, %rdx
	sarq	$33, %rcx
	addl	%edx, %ecx
	leal	(%rcx,%rcx,4), %ecx
	subl	%ecx, %eax
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$0, -16(%rax)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	cmpl	$0, -16(%rcx)
	sete	-16(%rax)
	jne	.LBB2_5
# %bb.4:                                # %label66
	callq	buzz@PLT
	jmp	.LBB2_2
.LBB2_5:                                # %label67
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$15, -16(%rax)
	movslq	-4(%rbp), %rax
	imulq	$-2004318071, %rax, %rcx        # imm = 0x88888889
	shrq	$32, %rcx
	addl	%eax, %ecx
	movl	%ecx, %edx
	shrl	$31, %edx
	sarl	$3, %ecx
	addl	%edx, %ecx
	leal	(%rcx,%rcx,4), %ecx
	leal	(%rcx,%rcx,2), %ecx
	subl	%ecx, %eax
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$0, -16(%rax)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	cmpl	$0, -16(%rcx)
	sete	-16(%rax)
	jne	.LBB2_7
# %bb.6:                                # %label87
	callq	fizz@PLT
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	callq	buzz@PLT
	jmp	.LBB2_2
.LBB2_7:                                # %label88
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$48, -16(%rax)
	movl	-4(%rbp), %edi
	addl	$48, %edi
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	%edi, -16(%rax)
	callq	putchar@PLT
.LBB2_2:                                # %label47
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$10, -16(%rax)
	movl	$10, %edi
	callq	putchar@PLT
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	xorl	%eax, %eax
	movq	%rbp, %rsp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end2:
	.size	judge, .Lfunc_end2-judge
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movl	$0, -4(%rbp)
	.p2align	4, 0x90
.LBB3_1:                                # %label110
                                        # =>This Inner Loop Header: Depth=1
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$30, -16(%rax)
	movl	-4(%rbp), %eax
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	cmpl	$30, %eax
	setl	-16(%rcx)
	cmpl	$29, %eax
	jg	.LBB3_3
# %bb.2:                                # %label111
                                        #   in Loop: Header=BB3_1 Depth=1
	movl	-4(%rbp), %edi
	callq	judge@PLT
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$1, -16(%rax)
	movl	-4(%rbp), %eax
	incl	%eax
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	movl	%eax, -4(%rbp)
	jmp	.LBB3_1
.LBB3_3:                                # %label113
	xorl	%eax, %eax
	movq	%rbp, %rsp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end3:
	.size	main, .Lfunc_end3-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
