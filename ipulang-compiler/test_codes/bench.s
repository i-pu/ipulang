	.text
	.file	"main"
	.globl	fib                             # -- Begin function fib
	.p2align	4, 0x90
	.type	fib,@function
fib:                                    # @fib
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%rbx
	subq	$24, %rsp
	.cfi_offset %rbx, -24
	movl	%edi, -16(%rbp)
	movl	$1, -20(%rbp)
	cmpl	$2, %edi
	setl	-9(%rbp)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	cmpl	$1, %edi
	jg	.LBB0_3
# %bb.1:                                # %label8
	movl	$1, -16(%rax)
	movl	$1, %eax
	jmp	.LBB0_2
.LBB0_3:                                # %label9
	movl	$1, -16(%rax)
	movl	-16(%rbp), %edi
	decl	%edi
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	%edi, -16(%rax)
	callq	fib@PLT
	movq	%rsp, %rbx
	leaq	-16(%rbx), %rsp
	movl	%eax, -16(%rbx)
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	$2, -16(%rax)
	movl	-16(%rbp), %edi
	addl	$-2, %edi
	movq	%rsp, %rax
	leaq	-16(%rax), %rsp
	movl	%edi, -16(%rax)
	callq	fib@PLT
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
	addl	-16(%rbx), %eax
	movq	%rsp, %rcx
	leaq	-16(%rcx), %rsp
	movl	%eax, -16(%rcx)
.LBB0_2:                                # %label8
	leaq	-8(%rbp), %rsp
	popq	%rbx
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Lfunc_end0:
	.size	fib, .Lfunc_end0-fib
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$42, 4(%rsp)
	movl	$42, %edi
	callq	fib@PLT
	movl	%eax, (%rsp)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
