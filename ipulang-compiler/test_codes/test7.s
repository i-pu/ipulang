	.text
	.file	"main"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movl	$72, 52(%rsp)
	movl	$72, %edi
	callq	putchar@PLT
	movl	%eax, 48(%rsp)
	movl	$69, 44(%rsp)
	movl	$69, %edi
	callq	putchar@PLT
	movl	%eax, 40(%rsp)
	movl	$76, 36(%rsp)
	movl	$76, %edi
	callq	putchar@PLT
	movl	%eax, 32(%rsp)
	movl	$76, 28(%rsp)
	movl	$76, %edi
	callq	putchar@PLT
	movl	%eax, 24(%rsp)
	movl	$79, 20(%rsp)
	movl	$79, %edi
	callq	putchar@PLT
	movl	%eax, 16(%rsp)
	movl	$33, 12(%rsp)
	movl	$33, %edi
	callq	putchar@PLT
	movl	%eax, 8(%rsp)
	movl	$0, 4(%rsp)
	xorl	%eax, %eax
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
