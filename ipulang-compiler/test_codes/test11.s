	.text
	.file	"main"
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
	movl	$1, -16(%rbp)
	movl	$1, -8(%rbp)
	movl	$10, -12(%rbp)
	movl	$10, -4(%rbp)
	movb	$1, %al
	.p2align	4, 0x90
.LBB0_1:                                # %label3
                                        # =>This Inner Loop Header: Depth=1
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	$0, -16(%rcx)
	testb	%al, %al
	jne	.LBB0_3
# %bb.2:                                # %label4
                                        #   in Loop: Header=BB0_1 Depth=1
	movl	-8(%rbp), %ecx
	addl	-4(%rbp), %ecx
	movq	%rsp, %rdx
	leaq	-16(%rdx), %rsp
	movl	%ecx, -16(%rdx)
	movl	%ecx, -8(%rbp)
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	$1, -16(%rcx)
	movl	-4(%rbp), %ecx
	incl	%ecx
	movq	%rsp, %rdx
	leaq	-16(%rdx), %rsp
	movl	%ecx, -16(%rdx)
	movl	%ecx, -4(%rbp)
	jmp	.LBB0_1
.LBB0_3:                                # %label6
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
