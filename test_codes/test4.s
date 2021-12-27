	.text
	.file	"main"
	.globl	a                               # -- Begin function a
	.p2align	4, 0x90
	.type	a,@function
a:                                      # @a
	.cfi_startproc
# %bb.0:                                # %entry
	movl	$0, -4(%rsp)
	xorl	%eax, %eax
	retq
.Lfunc_end0:
	.size	a, .Lfunc_end0-a
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	movl	$6, -4(%rsp)
	movl	$6, -8(%rsp)
	movl	$2, -12(%rsp)
	movl	$8, -16(%rsp)
	movl	$4, -20(%rsp)
	movl	$1, -24(%rsp)
	movl	$4, -28(%rsp)
	movl	$32, -32(%rsp)
	movl	$32, -36(%rsp)
	movl	$32, %eax
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
