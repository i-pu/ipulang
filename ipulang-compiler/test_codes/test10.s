	.text
	.file	"main"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	movl	$1, -4(%rsp)
	movl	$2, -8(%rsp)
	movb	$1, -9(%rsp)
	xorl	%eax, %eax
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
