	.text
	.file	"main"
	.globl	f                               # -- Begin function f
	.p2align	4, 0x90
	.type	f,@function
f:                                      # @f
	.cfi_startproc
# %bb.0:                                # %entry
	movl	%edi, %eax
	movl	%edi, -4(%rsp)
	movl	%esi, -8(%rsp)
	addl	%esi, %eax
	movl	%eax, -12(%rsp)
	retq
.Lfunc_end0:
	.size	f, .Lfunc_end0-f
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movl	$1, 36(%rsp)
	movl	$0, 32(%rsp)
	movl	$1, %edi
	xorl	%esi, %esi
	callq	f@PLT
	movl	%eax, 12(%rsp)
	movl	$3, 28(%rsp)
	movl	$9, 24(%rsp)
	movl	$3, %edi
	movl	$9, %esi
	callq	f@PLT
	movl	%eax, 20(%rsp)
	addl	12(%rsp), %eax
	movl	%eax, 16(%rsp)
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
