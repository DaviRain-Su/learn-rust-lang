# Fundations

As you dive into the more advanced corners of Rust, it’s important that you ensure you have a solid understanding of the fundamentals. In Rust, as in any programming language, the precise meaning of various keywords and concepts becomes important as you begin to use the language in more sophisticated ways. In this chapter, we’ll walk through many of Rust’s primitives and try to define more clearly what they mean, how they work, and why they are exactly the way that they are. Specifically, we’ll look at how variables and values differ, how they are represented in memory, and the different memory regions a program has. We’ll then discuss some of the subtleties of ownership, borrowing, and lifetimes that you’ll need to have a handle on before you continue with the book.

​    在你深入 Rust 更高级的领域之前，确保你对基础知识有坚实的理解非常重要。在 Rust 中，就像在任何编程语言中一样，*随着你开始以更复杂的方式使用该语言，各种关键字和概念的确切含义变得更加重要*。在本章中，我们将介绍许多 Rust 的基本元素，并尝试更清晰地定义它们的含义、工作原理以及它们为什么会是现在这个样子。具体而言，我们将**了解变量和值的区别、它们在内存中的表示方式以及程序所具有的不同内存区域**。然后，我们将讨论一些关于**所有权、借用和生命周期**的微妙之处，这些是你在继续本书之前需要掌握的知识。

You can read this chapter from top to bottom if you wish, or you can use it as a reference to brush up on the concepts that you feel less
sure about. I recommend that you move on only when you feel completely comfortable with the content of this chapter, as
misconceptions about how these primitives work will quickly get in the way of understanding the more advanced topics, or lead to you using them incorrectly.

​  你可以按照顺序从头到尾阅读这一章，或者将其作为参考，了解你不确定的概念。我建议在你完全理解本章内容之前不要继续，**因为对这些基本原理的错误理解会很快妨碍你理解更高级的主题**，或导致你错误地使用它们。

## Taling About Memory

Not all memory is created equal. In most programming environments, your programs have access to a stack, a heap,
registers, text segments, memory-mapped registers, memory-mapped files, and perhaps nonvolatile RAM. Which one you choose
to use in a particular situation has implications for what you can store there, how long it remains accessible, and what mechanisms
you use to access it. The exact details of these memory regions vary between platforms and are beyond the scope of this book, but some
are so important to how you reason about Rust code that they are worth covering here

并非所有的内存都是相同的。在大多数编程环境中，程序可以访问栈、堆、寄存器、文本段、内存映射寄存器、内存映射文件，以及可能的非易失性RAM。在特定情况下选择哪种内存区域会对可存储的内容、访问时长以及访问机制产生影响。这些内存区域的确切细节因平台而异，超出了本书的范围，但其中一些对于理解 Rust 代码非常重要，因此值得在这里介绍。

### Memory Terminology

Before we dive into regions of memory, you first need to know about
the difference between values, variables, and pointers. A value in
Rust is the combination of a type and an element of that type’s
domain of values. A value can be turned into a sequence of bytes
using its type’s representation, but on its own you can think of a
value more like what you, the programmer, meant. For example, the
number `6` in the type `u8` is an instance of the mathematical integer 6,
and its in-memory representation is the byte `0x06`. Similarly, the `str`
`"Hello world"` is a value in the domain of all strings whose
representation is its UTF-8 encoding. A value’s meaning is
independent of the location where those bytes are stored.

在深入探讨内存区域之前，您需要**了解值、变量和指针之间的区别**。在 Rust 中，值是类型和该类型值域的元素的组合。可以使用其类型的表示将值转换为字节序列，但单独来看，您可以将值视为程序员的意图。例如，类型为 `u8` 的数字 `6` 是数学整数 `6` 的实例，其内存表示为字节 `0x06`。类似地，字符串 `"Hello world"` 是所有字符串域中的值，其表示形式是其 UTF-8 编码。**值的含义与存储这些字节的位置无关。**

A value is stored in a place, which is the Rust terminology for “a
location that can hold a value.” This place can be on the stack, on the
heap, or in a number of other locations. The most common place to
store a value is a variable, which is a named value slot on the stack.

*值存储在位置中，这是 Rust 中“可以容纳一个值的位置”的术语。* 这个位置可以在堆栈(stack)、堆或其他许多位置上。存储值的最常见位置是变量，它是堆栈上的一个命名值槽。

A pointer is a value that holds the address of a region of memory,
so the pointer points to a place. A pointer can be dereferenced to
access the value stored in the memory location it points to. We can
store the same pointer in more than one variable and therefore have
multiple variables that indirectly refer to the same location in
memory and thus the same underlying value.

指针是一个保存了某个内存区域地址的值，因此指针指向一个位置。可以通过解引用指针来访问内存位置中存储的值。可以将相同的指针存储在多个变量中，从而拥有多个间接地指向内存中相同位置的变量，也就是拥有同一个基础值。

Consider the code in Listing 1-1, which illustrates these three
elements.

考虑清单1-1中的代码，它说明了这三个要素。


```rust
let x = 42;
let y = 43;
let var1 = &x;
let mut var2 = &x;
1: var2 = &y;
```

Listing 1-1: Values, variables, and pointers


Here, there are four distinct values: 42 (an i32), 43 (an i32), the
address of x (a pointer), and the address of y (a pointer). There are
also four variables: x, y, var1, and var2. The latter two variables both
hold values of the pointer type, because references are pointers.
While var1 and var2 store the same value initially, they store separate,
independent copies of that value; when we change the value stored in
var2 1, the value in var1 does not change. In particular, the = operator
stores the value of the right-hand side expression in the place named
by the left-hand side.

这里有四个不同的值：`42`（一个i32类型的整数）、`43`（一个i32类型的整数）、`x`的地址（一个指针）和`y`的地址（一个指针）。还有四个变量：`x`、`y`、`var1`和`var2`。后两个变量都持有指针类型的值，因为引用就是指针。虽然`var1`和`var2`最初存储相同的值，但它们存储独立的、互不干扰的副本；当我们改变var2中存储的值时，var1中的值不会改变。特别地，赋值操作符“=”会将右侧表达式的值存储到左侧命名的位置。

An interesting example of where the distinction between variables,
values, and pointers becomes important is in a statement such as:

变量、值和指针之间的区别变得很重要的一个有趣的例子是在这样的语句中：

```rust
let string = "Hello, world";
```

Even though we assign a string value to the variable string, the
actual value of the variable is a pointer to the first character in the
string value "Hello world", and not the string value itself. At this point
you might say, “But hang on, where is the string value stored, then?
Where does the pointer point?” If so, you have a keen eye—we’ll get
to that in a second.

值得注意的是，尽管我们将一个字符串值赋给变量 string，但实际上变量的值是一个指向字符串值 "Hello world" 的第一个字符的指针，而不是字符串值本身。此时你可能会问，“但是等等，那么字符串值存储在哪里呢？指针指向哪里？”如果是这样，那么你有一双敏锐的眼睛——我们很快就会讨论这个问题。

> Note: Technically, the value of string also includes the string’s
length. We’ll talk about that in Chapter 2 when we discuss
wide pointer types.

> Note: 技术上讲，string 的值还包括字符串的长度。我们将在第2章讨论宽指针类型时详细讨论这一点。

### Variables in Depth

The definition of a variable I gave earlier is broad and unlikely to be
all that useful in and of itself. As you encounter more complex code,
you’ll need a more accurate mental model to help you reason through
what the programs are really doing. There are many such models
that we can make use of. Describing them all in detail would take up
several chapters and is beyond the scope of this book, but broadly
speaking, they can be divided into two categories: high-level models
and low-level models. High-level models are useful when thinking
about code at the level of lifetimes and borrows, while low-level
models are good for when you are reasoning about unsafe code and
raw pointers. The models for variables described in the following two
sections will suffice for most of the material in this book

我之前给出的变量定义是广泛的，本身可能不是很有用。随着你遇到更复杂的代码，你需要一个更准确的思维模型来帮助你理解程序的实际运行情况。我们可以利用许多这样的模型。详细描述所有这些模型需要几章的篇幅，已超出本书的范围，但大体上可以分为两类：**高级模型和低级模型。** **高级模型在考虑代码的生命周期和借用级别时很有用，而低级模型则适用于在推理不安全代码和原始指针时。** 下面两节描述的变量模型足以满足本书大部分内容的需要。

#### 高级模型（Hight-Level Model）

In the high-level model, we don’t think of variables as places that
hold bytes. Instead, we think of them just as names given to values as
they are instantiated, moved, and used throughout a program. When
you assign a value to a variable, that value is from then on named by
that variable. When a variable is later accessed, you can imagine
drawing a line from the previous access of that variable to the new
access, which establishes a dependency relationship between the two
accesses. If the value in a variable is moved, no lines can be drawn
from it anymore.

**在高级模型中，我们不认为变量是保存字节的位置。相反，我们将它们视为在程序中实例化、移动和使用时赋予值的名称。** 当你将一个值赋给一个变量时，从那时起，该变量就代表了该值。当稍后访问变量时，可以想象从该变量的先前访问到新访问之间绘制一条线，这样就建立了两个访问之间的依赖关系。如果变量中的值被移动了，就无法再从它上面绘制线条了。

In this model, a variable exists only so long as it holds a legal
value; you cannot draw lines from a variable whose value is
uninitialized or has been moved, so effectively it isn’t there. Using
this model, your entire program consists of many of these
dependency lines, often called flows, each one tracing the lifetime of
a particular instance of a value. Flows can fork and merge when there
are branches, with each split tracing a distinct lifetime for that value.

*在这个模型中，变量只存在于它持有合法值的期间；你不能从一个值未初始化或已被移动的变量上绘制线条，因此实际上它不存在。* 使用这个模型，你的整个程序由许多这些依赖线条组成，通常被称为流，每个流追踪一个特定值实例的生命周期。当出现分支时，流可以分叉和合并，每个分支追踪该值的不同生命周期。


The compiler can check that at any given point in your program, all
flows that can exist in parallel with each other are compatible. For
example, there cannot be two parallel flows with mutable access to a
value. Nor can there be a flow that borrows a value while there is no
flow that owns the value. Listing 1-2 shows examples of both of these
cases.

**编译器可以检查程序中任何给定点，所有可以并行存在的流都是兼容的。** 例如，**不能有两个并行的流对一个值具有可变访问权限。** 同样，不能有一个流借用一个值，而没有拥有该值的流存在。代码清单 1-2 展示了这两种情况的示例。


```rust
let mut x;
// this access would be illegal, nowhere to draw the flow from:
// 这条通道将是非法的，没有地方可以引流：
// 这里是非法的，这里的x在assert_eq中用的时候x还没有初始化，所以他是不存在的。
// assert_eq!(x, 42);
1: x = 42;
// this is okay, can draw a flow from the value assigned above:
// 这是可以的，可以从上面分配的值中画出一个流程。
2: let y = &x;
// this establishes a scond, mutable flow from x:
// 这就建立了一个来自x的第二个可变的流。
3: x = 43;
// this continus the flow from y, wich in turn draws from x.
// but that flow conflicts with the assignment to x!
// 这就继续了来自y的流量，而y又从x中提取。但这一流动与分配给X的任务相冲突!
4: assert_eq!(*y,42);
```

Listing 1-2: Illegal flows that the borrow checker will catch

First, we cannot use `x` before it is initialized, because we have
nowhere to draw the flow from. Only when we assign a value to x can
we draw flows from it. This code has two flows: one exclusive (&mut)
flow from 1 to 3, and one shared (&) flow from 1 through 2 to 4.
The borrow checker inspects every vertex of every flow and checks
that no other incompatible flows exist concurrently. In this case,
when the borrow checker inspects the exclusive flow at 3, it sees the
shared flow that terminates at 4. Since you cannot have an exclusive
and a shared use of a value at the same time, the borrow checker
(correctly) rejects the code. Notice that if 4 was not there, this code
would compile fine! The shared flow would terminate at 2, and
when the exclusive flow is checked at 3, no conflicting flows would
exist.

首先，我们不能在初始化 `x` 之前使用它，因为我们没有任何地方可以绘制流。只有当我们给 `x` 赋值时，才能从它绘制流。这段代码有两个流：一个从 `1` 到 `3` 的排他（&mut）流，以及一个从 `1` 经过 `2` 到 `4` 的共享（&）流。**借用检查器检查每个流的每个顶点，并检查是否存在其他不兼容的流并发存在。** 在这种情况下，当借用检查器检查 `3` 处的排他流时，它看到终止于 `4` 的共享流。由于您不能同时具有值的排他和共享使用，因此借用检查器（正确地）拒绝该代码。请注意，如果不存在 `4`，这段代码将编译良好！共享流将在 `2` 处终止，当检查 `3` 处的排他流时，不存在冲突的流。

If a new variable is declared with the same name as a previous one,
they are still considered distinct variables. This is called shadowing
—the later variable “shadows” the former by the same name. The two
variables coexist, though subsequent code no longer has a way to
name the earlier one. This model matches roughly how the compiler,
and the borrow checker in particular, reasons about your program,
and is actually used internally in the compiler to produce efficient
code.

如果使用与先前变量相同的名称声明新变量，则它们仍然被视为不同的变量。这被称为“遮蔽”——后面的变量通过相同的名称“遮蔽”了前面的变量。这两个变量共存，尽管随后的代码不再有方法来命名先前的变量。这个模型大致匹配编译器（特别是借用检查器）对程序进行推理的方式，实际上在编译器内部用于生成高效的代码。

#### 低级模型(Low-Level Model)

Variables name memory locations that may or may not hold legal
values. You can think of a variable as a “value slot.” When you assign
to it, the slot is filled, and its old value (if it had one) is dropped and
replaced. When you access it, the compiler checks that the slot isn’t
empty, as that would mean the variable is uninitialized or its value
has been moved. A pointer to a variable refers to the variable’s
backing memory and can be dereferenced to get at its value. For
example, in the statement let x: usize, the variable x is a name for a
region of memory on the stack that has room for a value the size of a
usize, though it does not have a well-defined value (its slot is empty).
If you assign a value to that variable, such as with x = 6, that region of
memory will then hold the bits representing the value 6. &x does not
change when you assign to x. If you declare multiple variables with
the same name, they still end up with different chunks of memory
backing them. This model matches the memory model used by C and
C++, and many other low-level languages, and is useful for when you
need to reason explicitly about memory.

变量命名可能保存合法值的内存位置。你可以将变量视为“值槽”。当你对它赋值时，槽被填充，它的旧值（如果有的话）被丢弃并替换。当你访问它时，编译器检查槽是否为空，因为这意味着变量未初始化或其值已移动。指向变量的指针指向变量的支持内存，可以通过对其进行取消引用来访问其值。例如，在语句 `let x: usize` 中，变量 `x` 是堆栈上的一个内存区域的名称，它具有大小为 `usize` 的值的空间，但它没有定义明确的值（它的槽为空）。如果你将一个值赋给该变量，例如 `x = 6`，那么内存中的该区域将保存表示值 `6` 的位。当你对 `x` 赋值时，`&x` 不会改变。如果你用相同的名称声明多个变量，它们仍然具有不同的内存块来支持它们。这个模型匹配` C` 和` C++` 以及许多其他低级语言使用的内存模型，对于需要显式推理内存的情况非常有用。

> note: In this example, we ignore CPU registers and treat them as
an optimization. In reality, the compiler may use a register
to back a variable instead of a region of memory if no
memory address is needed for that variable.

> Note: 在这个例子中，我们忽略了 CPU 寄存器并将它们视为一种优化。实际上，如果一个变量不需要内存地址，编译器可能会使用寄存器来支持变量，而不是内存区域。

You may find that one of these matches your previous model better
than the other, but I urge you to try to wrap your head around both
of them. They are both equally valid, and both are simplifications,
like any useful mental model has to be. If you are able to consider a
piece of code from both of these perspectives, you will find it much
easier to work through complicated code segments and understand
why they do or do not compile and work as you expect.

你可能会发现其中一种模型比另一种更符合你以前的模型，但我敦促你尝试理解两者。它们都是同样有效的，都是简化，就像任何有用的思维模型一样。如果你能从这两个角度考虑代码片段，你会发现更容易解决复杂的代码段，并理解它们是否符合你的期望以及为什么会出现编译问题。

### Memory Regions

Now that you have a grip on how we refer to memory, we need to talk
about what memory actually is. There are many different regions of
memory, and perhaps surprisingly, not all of them are stored in the
DRAM of your computer. Which part of memory you use has a
significant impact on how you write your code. The three most
important regions for the purposes of writing Rust code are the
stack, the heap, and static memory.

现在你已经了解了我们如何引用内存，我们需要讨论一下内存实际上是什么。内存有许多不同的区域，也许令人惊讶的是，并不是所有的内存区域都存储在计算机的DRAM中。你使用的内存部分对你编写代码的方式有很大的影响。**为了编写 Rust 代码，最重要的三个内存区域是堆栈、堆和静态内存。**

#### The Stack

The stack is a segment of memory that your program uses as scratch
space for function calls. Each time a function is called, a contiguous
chunk of memory called a frame is allocated at the top of the stack.
Near the bottom of the stack is the frame for the main function, and as
functions call other functions, additional frames are pushed onto the
stack. A function’s frame contains all the variables within that
function, along with any arguments the function takes. When the
function returns, its stack frame is reclaimed.

堆栈是程序用作函数调用的临时空间的一段内存。每次调用一个函数时，一个称为帧的连续内存块被分配在堆栈的顶部。在堆栈的底部附近是主函数的帧，在函数调用其他函数时，附加的帧被推入堆栈。一个函数的帧包含该函数内的所有变量，以及该函数接受的任何参数。当函数返回时，它的堆栈帧被回收。

The bytes that make up the values of the function’s local variables
are not immediately wiped, but it’s not safe to access them as they
may have been overwritten by a subsequent function call whose
frame overlaps with the reclaimed one. And even if they haven’t been
overwritten, they may contain values that are illegal to use, such as
ones that were moved when the function returned.

函数的本地变量的值所占的字节并不会立即被清除，但是访问它们是不安全的，因为它们可能已被重写为后续函数调用的帧与被回收的帧重叠。即使它们没有被重写，它们也可能包含不合法的值，例如在函数返回时已被移动的值。

Stack frames, and crucially the fact that they eventually disappear,
are very closely tied to the notion of lifetimes in Rust. Any variable
stored in a frame on the stack cannot be accessed after that frame
goes away, so any reference to it must have a lifetime that is at most
as long as the lifetime of the frame.

堆栈帧，以及它们最终消失的事实，与 Rust 中的生命周期概念密切相关。在堆栈的帧上存储的任何变量在该帧消失后都无法访问，因此对它的任何引用的生命周期必须最长不超过该帧的生命周期。

#### The Heap

The heap is a pool of memory that isn’t tied to the current call stack
of the program. Values in heap memory live until they are explicitly
deallocated. This is useful when you want a value to live beyond the
lifetime of the current function’s frame. If that value is the function’s
return value, the calling function can leave some space on its stack
for the called function to write that value into before it returns. But if
you want to, say, send that value to a different thread with which the
current thread may share no stack frames at all, you can store it on
the heap.

堆是一个内存池，它与程序的当前调用堆栈没有关系。在堆内存中的值一直存在，直到它们被明确释放。当你想让一个值存活在当前函数帧的生命周期之外时，这非常有用。如果该值是函数的返回值，调用函数可以在其堆栈上留出一些空间供被调用函数在返回之前写入该值。但是，如果你想将该值发送到一个不与当前线程共享任何堆栈帧的不同线程中，你可以将其存储在堆中。

The heap allows you to explicitly allocate contiguous segments of
memory. When you do so, you get a pointer to the start of that
segment of memory. That memory segment is reserved for you until
you later deallocate it; this process is often referred to as freeing,
after the name of the corresponding function in the C standard
library. Since allocations from the heap do not go away when a
function returns, you can allocate memory for a value in one place,
pass the pointer to it to another thread, and have that thread safely
continue to operate on that value. Or, phrased differently, when you
heap-allocate memory, the resulting pointer has an unconstrained
lifetime—its lifetime is however long your program keeps it alive.

堆允许你显式地分配连续的内存段。当你这样做时，你会得到指向该内存段开头的指针。该内存段为你保留，直到你以后释放它；这个过程通常被称为释放，以 C 标准库中相应函数的名称命名。由于从堆中分配的内存不会在函数返回时消失，你可以在一个地方为值分配内存，将指针传递给另一个线程，并让该线程安全地继续操作该值。或者换句话说，当你从堆中分配内存时，所得到的指针具有无约束的生命周期——它的生命周期就是你的程序将其保持活动的时间。

The primary mechanism for interacting with the heap in Rust is
the `Box` type. When you write `Box::new(value)`, the value is placed on the
heap, and what you are given back (the `Box<T>`) is a pointer to that
value on the heap. When the `Box` is eventually dropped, that memory
is freed.

在 Rust 中与堆交互的主要机制是 `Box` 类型。当你编写 `Box::new(value)` 时，该值被放置在堆上，你所得到的是指向堆上该值的指针（`Box<T>`）。当 `Box` 最终被 dropped 时，该内存被释放。

If you forget to deallocate heap memory, it will stick around
forever, and your application will eventually eat up all the memory
on your machine. This is called leaking memory and is usually
something you want to avoid. However, there are some cases where
you explicitly want to leak memory. For example, say you have a
read-only configuration that the entire program should be able to
access. You can allocate that on the heap and explicitly leak it with
`Box::leak` to get a `'static` reference to it.

如果你忘记释放堆内存，它将永远存在，并且你的应用程序最终会耗尽机器上的所有内存。这被称为内存泄漏，通常是你要避免的情况。然而，有些情况下你明确地想要泄漏内存。例如，假设你有一个只读配置，整个程序都应该能够访问它。你可以将其分配在堆上，并使用 `Box::leak` 显式泄漏它，以获取对它的 `'static` 引用。

#### Static Memory

Static memory is really a catch-all term for several closely related
regions located in the file your program is compiled into. These
regions are automatically loaded into your program’s memory when
that program is executed. Values in static memory live for the entire
execution of your program. Your program’s static memory contains
the program’s binary code, which is usually mapped as read-only. As
your program executes, it walks through the binary code in the text
segment instruction by instruction and jumps around whenever a
function is called. Static memory also holds the memory for variables
you declare with the static keyword, as well as certain constant values
in your code, like strings.

静态内存实际上是一个包含在你的程序编译后的文件中的几个密切相关的区域的综合术语。当程序被执行时，这些区域会自动加载到程序的内存中。**静态内存中的值在整个程序的执行期间都存在。** 你的程序的静态内存包含程序的二进制代码，通常被映射为只读。当你的程序执行时，它会逐条指令地在文本段中遍历二进制代码，并在每次调用函数时跳转。**静态内存还包含用 `static` 关键字声明的变量的内存，以及你代码中的某些常量值，如字符串。**

The special lifetime 'static, which gets its name from the static
memory region, marks a reference as being valid for “as long as static
memory is around,” which is until the program shuts down. Since a
static variable’s memory is allocated when the program starts, a
reference to a variable in static memory is, by definition, 'static, as it
is not deallocated until the program shuts down. The inverse is not
true—there can be 'static references that do not point to static
memory—but the name is still appropriate: once you create a
reference with a static lifetime, whatever it points to might as well be
in static memory as far as the rest of the program is concerned, as it
can be used for however long your program wishes.

特殊的生命周期`'static'`得名于静态内存区域，它标记了一个引用为“只要静态内存存在”，也就是直到程序关闭。由于静态变量的内存是在程序启动时分配的，对静态内存中变量的引用是`'static'`的，因为它直到程序关闭才被释放。反之不成立——可能存在指向非静态内存的`'static'`引用——但名称仍然适用：一旦创建了具有`'static'`生命周期的引用，无论它指向什么，就像在静态内存中一样，就程序的其余部分而言，它可以被使用任意长的时间。

You will encounter the 'static lifetime much more often than you
will encounter truly static memory (through the static keyword, for
example) when working with Rust. This is because 'static often shows
up in trait bounds on type parameters. A bound like T: 'static
indicates that the type parameter T is able to live for however long we
keep it around for, up to and including the remaining execution of
the program. Essentially, this bound requires that T is owned and
self-sufficient, either in that it does not borrow other (non-static)
values or that anything it does borrow is also 'static and thus will
stick around until the end of the program. A good example of 'static
as a bound is the std::thread::spawn function that creates a new thread,
which requires that the closure you pass it is 'static. Since the new
thread may outlive the current thread, the new thread cannot refer to
anything stored on the old thread’s stack. The new thread can refer
only to values that will live for its entire lifetime, which may be for
the remaining duration of the program.

在Rust中，相比真正的静态内存（例如通过`static`关键字），你会更经常遇到`'static`生命周期。这是因为`'static`常常出现在类型参数的`trait bound`中。例如，像`T: 'static`这样的绑定表示类型参数`T`能够在我们保留它的时间内存活，直到程序执行结束。本质上，这个绑定要求`T`是自主拥有的，要么它不借用其他（非静态）值，要么它借用的任何值也都是`'static`的，因此将一直存在直到程序结束。一个很好的`'static`作为`bound`的例子是`std::thread::spawn`函数，它创建一个新的线程，需要传递给它的闭包是`'static`的。由于新线程可能会超出当前线程的生命周期，新线程不能引用存储在旧线程堆栈上的任何东西。新线程只能引用将在其整个生命周期内存活的值，这可能是程序剩余的持续时间。

> Note: You may wonder how const differs from static. The const
keyword declares the following item as constant. Constant
items can be completely computed at compile time, and any
code that refers to them is replaced with the constant’s
computed value during compilation. A constant has no
memory or other storage associated with it (it is not a place).
You can think of constant as a convenient name for a
particular value


> Note: 您可能会想知道`const`与`static`的区别。`const`关键字声明以下项为常量。常量可以在编译时完全计算，任何引用它们的代码都会在编译期间被替换为常量的计算值。**常量没有与其关联的内存或其他存储（它不是一个位置）。您可以将常量视为特定值的方便名称。**

## Ownership


Rust’s memory model centers on the idea that all values have a single
owner—that is, exactly one location (usually a scope) is responsible
for ultimately deallocating each value. This is enforced through the
borrow checker. If the value is moved, such as by assigning it to a
new variable, pushing it to a vector, or placing it on the heap, the
ownership of the value moves from the old location to the new one.
At that point, you can no longer access the value through variables
that flow from the original owner, even though the bits that make up
the value are technically still there. Instead, you must access the
moved value through variables that refer to its new location.

Rust的内存模型围绕着这样一个思想：所有值都有唯一的所有者——也就是说，每个值最终释放的责任都归属于一个位置（通常是一个作用域）。这是通过借用检查器实现的。如果值被移动，例如通过将其赋值给一个新变量、将其推送到向量中或将其放置在堆上，则该值的所有权从旧位置移动到新位置。此时，您不能再通过从原始所有者流动的变量访问该值，即使构成该值的位仍然存在。相反，您必须通过引用其新位置的变量来访问已移动的值。

Some types are rebels and do not follow this rule. If a value’s type
implements the special Copy trait, the value is not considered to have
moved even if it is reassigned to a new memory location. Instead, the
value is copied, and both the old and new locations remain
accessible. Essentially, another identical instance of that same value
is constructed at the destination of the move. Most primitive types in
Rust, such as the integer and floating-point types, are Copy. To be Copy,
it must be possible to duplicate the type’s values simply by copying
their bits. This eliminates all types that contain non-Copy types as well
as any type that owns a resource it must deallocate when the value is
dropped

有些类型是叛逆的，不遵循这个规则。如果一个值的类型实现了特殊的`Copy trait`，即使它被重新分配到新的内存位置，该值也不被认为已经移动。相反，该值被复制，旧位置和新位置都仍然可以访问。本质上，另一个完全相同的该值的实例在移动的目标处被构造。Rust中的大多数原始类型，例如整数和浮点数类型，都是`Copy`。为了实现`Copy`，必须能够仅通过复制它们的位来复制类型的值。这消除了所有包含非`Copy`类型的类型，以及任何在值被删除时必须释放资源的类型的所有权

To see why, consider what would happen if a type like Box were Copy.
If we executed box2 = box1, then box1 and box2 would both believe that
they owned the heap memory allocated for the box, and they would
both attempt to free it when they went out of scope. Freeing the
memory twice could have catastrophic consequences.

为了理解为什么这样做不可行，请考虑如果像`Box`这样的类型是`Copy`会发生什么。如果我们执行`box2 = box1`，那么`box1`和`box2`都会认为它们拥有为该`Box`分配的堆内存，并且它们都会在作用域结束时尝试释放它。释放内存两次可能会产生灾难性的后果。

When a value’s owner no longer has use for it, it is the owner’s
responsibility to do any necessary cleanup for that value by dropping
it. In Rust, dropping happens automatically when the variable that
holds the value is no longer in scope. Types usually recursively drop
values they contain, so dropping a variable of a complex type may
result in many values being dropped. Because of Rust’s discrete
ownership requirement, we cannot accidentally drop the same value
multiple times. A variable that holds a reference to another value
does not own that other value, so the value isn’t dropped when the
variable drops.

当一个值的所有者不再使用它时，所有权者有责任通过丢弃该值来执行任何必要的清理工作。在Rust中，当保存值的变量不再在作用域内时，自动进行丢弃。类型通常会递归地丢弃它们包含的值，因此丢弃复杂类型的变量可能导致许多值被丢弃。由于Rust的**离散所有权**要求，我们不能意外地多次丢弃相同的值。保存对另一个值的引用的变量不拥有该其他值，因此在变量丢弃时该值不会被丢弃。

The code in Listing 1-3 gives a quick summary of the rules around
ownership, move and copy semantics, and dropping.

清单1-3中的代码对围绕所有权、移动和复制语义以及放弃的规则进行了快速总结

```rust
let x1 = 42;
let y1 = Box::new(84);
{
  // startsa new scope
  1: let z = (x1, y1);
  // z goes out of scope, and is dropes:
  // it in turn drops the values from x1 and y1
2: }
// x1's value is Copy, so it was not moved into z
3: let x2 = x1;
// y1's value is not Copy, so it was moved into z
// let y2 = y1;
```

Listing 1-3: Moving and copying semantics

We start out with two values, the number 42 and a Box (a heap-allocated value) containing the number 84. The former is Copy, whereas
the latter is not. When we place x1 and y1 into the tuple z 1, x1 is
copied into z, whereas y1 is moved into z. At this point, x1 continues to
be accessible and can be used again 3. On the other hand, y1 is
rendered inaccessible once its value has been moved 4, and any
attempt to access it would incur a compiler error. When z goes out of
scope 2, the tuple value it contains is dropped, and this in turn
drops the value copied from x1 and the one moved from y1. When the
Box from y1 is dropped, it also deallocates the heap memory used to
store y1’s value.

我们从两个值开始，一个是数字`42`，另一个是包含数字`84`的`Box`（一个堆分配的值）。前者是`Copy`，而后者不是。当我们将`x1`和`y1`放入元组`z1`中时，`x1`被复制到`z`中，而`y1`被移动到`z`中。此时，`x1`仍然可以访问并且可以再次使用`3`。另一方面，`y1`的值一旦被移动，就变得无法访问`4`，任何尝试访问它的尝试都会导致编译器错误。当`z`退出作用域`2`时，它包含的元组值被丢弃，这进而使从`x1`复制的值和从`y1`移动的值都被丢弃。当从`y1`中丢弃`Box`时，它也会释放用于存储`y1`值的堆内存。


> Drop Order
>
> Rust automatically drops values when they go out of scope, such as x1 and y1 in the inner scope in Listing 1-3. The rules for the order in which to drop are fairly simple: variables (including function arguments) are dropped in reverse order, and nested values are dropped in source-code order.
>
>Rust在值超出其作用域时自动丢弃这些值，例如清单1-3中内部作用域中的`x1`和`y1`。**按照丢弃顺序的规则相当简单：变量（包括函数参数）按照相反的顺序进行丢弃，嵌套值按照源代码顺序进行丢弃**
>
>This might sound weird at first—why the discrepancy? If we look at it closely, though, it makes a lot of sense. Say you write a function that declares a string and then inserts a reference to that string into a new hash table. When the function returns, the hash table must be dropped first; if the string were dropped first, the hash table would then hold an invalid reference! In general, later variables may contain references to earlier values, whereas the inverse cannot happen due to Rust’s lifetime rules. And for that reason, Rust drops variables in reverse order.
>
>这可能一开始听起来很奇怪——为什么会有这样的不一致性？但是，仔细看看，它有很多意义。假设您编写一个函数，声明一个字符串，然后将对该字符串的引用插入到一个新的哈希表中。当函数返回时，必须首先删除哈希表；如果先删除字符串，那么哈希表将持有一个无效引用！通常，后面的变量可能包含对先前值的引用，而逆反情况由于`Rust`的生命周期规则而无法发生。因此，Rust以相反的顺序删除变量。
>
>now, we could have the same behavior for nested values, like the values in a tuple, array, or struct, but that would likely surprise users. If you constructed an array that contained two values, it’d seem odd if the last element of the array were dropped first. The same applies to tuples and structs, where the most intuitive behavior is for the first tuple element or field to be dropped first, then the second, and so on. Unlike for variables, there is no need to reverse the drop order in this case, since Rust doesn’t (currently) allow self-references in a single value. So, Rust goes with the intuitive option.
>
>我们可以对嵌套值采用相同的行为，例如元组、数组或结构中的值，但是这可能会让用户感到惊讶。如果您构建一个包含两个值的数组，如果数组的最后一个元素首先被丢弃，那么它看起来很奇怪。对于元组和结构也是一样，最直观的行为是首先丢弃第一个元组元素或字段，然后是第二个元素或字段，依此类推。**与变量不同，在这种情况下没有必要反转丢弃顺序，因为Rust（目前）不允许单个值中的自引用。因此，Rust采用了直观的选项。**


## Borrowing and Lifetimes

Rust allows the owner of a value to lend out that value to others,
without giving up ownership, through references. References are
pointers that come with an additional contract for how they can be
used, such as whether the reference provides exclusive access to the
referenced value, or whether the referenced value may also have
other references point to it

Rust允许值的所有者通过引用将该值借出给其他人，而不放弃所有权。**引用是带有附加使用约定的指针，例如引用是否提供对引用值的独占访问权，或者引用值是否也可以有其他引用指向它。**

### Shared References

A shared reference, &T, is, as the name implies, a pointer that may be
shared. Any number of other references may exist to the same value,
and each shared reference is Copy, so you can trivially make more of
them. Values behind shared references are not mutable; you cannot
modify or reassign the value a shared reference points to, nor can
you cast a shared reference to a mutable one.

一个共享引用 `&T`，正如名称所示，是一个可以共享的指针。可以存在对同一值的任意数量的其他引用，每个共享引用都是`Copy`，因此您可以轻松地创建更多的引用。**共享引用背后的值不可变；您不能修改或重新分配共享引用指向的值，也不能将共享引用强制转换为可变引用。**

The Rust compiler is allowed to assume that the value a shared
reference points to will not change while that reference lives. For
example, if the Rust compiler sees that the value behind a shared
reference is read multiple times in a function, it is within its rights to
read it only once and reuse that value. More concretely, the assertion
in Listing 1-4 should never fail.

**Rust编译器可以假设共享引用指向的值在该引用存在期间不会更改。** 例如，如果Rust编译器看到在函数中多次读取共享引用后面的值，则可以合法地仅读取一次并重复使用该值。更具体地说，在清单1-4中的断言不应失败。

```rust
fn cache(input: &i32, sum: &mut i32) {
  *sum = *input + *input;
  assert_eq!(*sum, 2* *input);
}
```

Listing 1-4: Rust assumes that shared references are immmutable


Whether or not the compiler chooses to apply a given optimization
is more or less irrelevant. The compiler heuristics change over time,
so you generally want to code against what the compiler is allowed to
do rather than what it actually does in a particular case at a
particular moment in time.

*无论编译器是否选择应用给定的优化，都更或多少无关紧要。编译器的启发式算法会随着时间而改变，因此您通常希望针对编译器允许执行的操作编写代码，而不是针对特定情况和特定时刻编译器实际执行的操作编写代码。*

### Mutable References

The alternative to a shared reference is a mutable reference: &mut T.
With mutable references, the Rust compiler is again allowed to make
full use of the contract that the reference comes with: the compiler
assumes that there are no other threads accessing the target value,
whether through a shared reference or a mutable one. In other
words, it assumes that the mutable reference is exclusive. This
enables some interesting optimizations that are not readily available
in other languages. Take, for example, the code in Listing 1-5

可变引用的替代方案是可变引用：`&mut T`。对于可变引用，`Rust`编译器再次可以充分利用引用所带来的约定：**编译器假设没有其他线程通过共享引用或可变引用访问目标值。** 换句话说，它假设可变引用是独占的。这使得一些有趣的优化在其他语言中不容易实现。例如，参见清单1-5中的代码。

```rust
// listing 1-5: Rust assumes that mutable references are exclusive.
fn nolias(input: &i32, output: &mut i32) {
  if *input == 1{
    1: *output = 2;
  }
  2: if *input != 1 {
    *output = 3;
  }
}
```
Listing 1-5: Rust assumes that mutable references are exclusive.


// todo
In Rust, the compiler can assume that input and output do not point
to the same memory. Therefore, the reassignment of output at 1
cannot affect the check at 2, and the entire function can be compiled
as a single if-else block. If the compiler could not rely on the exclusive
mutability contract, that optimization would be invalid, since an input
of 1 could then result in an output of 3 in a case like noalias(&x, &mut x).

在Rust中，编译器可以假设输入和输出不指向同一内存。因此，1处输出的重新分配不能影响2处的检查，整个函数可以被编译为一个单一的if-else块。如果编译器不能依赖排他性可变性契约，那么这种优化就会失效，因为在noalias(&x, &mut x)这样的情况下，1的输入可能导致3的输出。


A mutable reference lets you mutate only the memory location that
the reference points to. Whether you can mutate values that lie
beyond the immediate reference depends on the methods provided
by the type that lies between. This may be easier to understand with
an example, so consider Listing 1-6.

一个可改变的引用只允许你改变该引用所指向的内存位置。你是否可以改变直接引用之外的值，取决于位于两者之间的类型所提供的方法。用一个例子可能更容易理解，所以考虑清单1-6。

```rust
// listing 1-6: Mutability applies only to the immediately referenced memory.

let x = 42;
let mut y = &x; // y is of type &i32
let z = &mut y; // z is of type &mut &i32
```

In this example, you are able to change the value of the pointer y to
a different value (that is, a different pointer) by making it reference a
different variable, but you cannot change the value that is pointed to
(that is, the value of x). Similarly, you can change the pointer value of
y through z, but you cannot change z itself to hold a different
reference.

在这个例子中，你能够通过让指针y引用一个不同的变量来改变它的值（也就是一个不同的指针），但你不能改变被指向的值（也就是x的值）。同样地，你可以通过z来改变y的指针值，但你不能改变z本身来持有不同的引用。

The primary difference between owning a value and having a
mutable reference to it is that the owner is responsible for dropping
the value when it is no longer necessary. Apart from that, you can do
anything through a mutable reference that you can if you own the
value, with one caveat: if you move the value behind the mutable
reference, then you must leave another value in its place. If you did
not, the owner would still think it needed to drop the value, but there
would be no value for it to drop!

拥有一个值和拥有一个可变的引用之间的主要区别是，所有者负责在不再需要的时候丢弃这个值。除此之外，你可以通过一个可改变的引用做任何事情，如果你拥有这个值的话，有一个注意事项：如果你把这个值移到可改变的引用后面，那么你必须在它的位置上留下另一个值。如果你不这样做，所有者仍然会认为它需要放弃这个值，但是它将没有值可以放弃！

Listing 1-7 gives an example of the ways in which you can move
the value behind a mutable reference

```rust
// listing 1-7 gives an example of the ways in which you can move the value behind a mutable reference.
// listing 1-7: Access through a mutable reference must leave a value behind.
fn replace_with_84(s: &mut Box<i32>) {
  // this is no okay, as *s would be empty:
  1: // let was = *s;
  // but this is:
  2: let was = std::mem::take(s);
  // so is this:
  3: *s = was;
  // we can exchange values behind &mut:
  let mut r = Box::new(84);
  4: std::mem::swap(s, &mut r);
  assert_ne!(*r, 84);
}
let mut s = Box::new(42);
replace_with_84(&mut s);
5: 
```

I’ve added commented-out lines that represent illegal operations.
You cannot simply move the value out 1 since the caller would still
think they owned that value and would free it again at 5, leading to
a double free. If you just want to leave some valid value behind,
std::mem::take 2 is a good candidate. It is equivalent to
std::mem::replace(&mut value, Default::default()); it moves value out from
behind the mutable reference but leaves a new, default value for the
type in its place. The default is a separate, owned value, so it is safe
for the caller to drop it when the scope ends at 5.

我已经添加了代表非法操作的注释行。你不能简单地将值移出1，因为调用者仍会认为他们拥有该值，并会在5处再次释放它，导致双重释放。如果你只是想留下一些有效的值，std::mem::take 2是一个不错的选择。它相当于std::mem::replace(&mut value, Default::default())；它将值从可变引用后面移出，但为该类型留下一个新的、默认的值。默认值是一个单独的、自有的值，所以当作用域在5处结束时，调用者可以安全地放弃它。

Alternatively, if you don’t need the old value behind the reference,
you can overwrite it with a value that you already own 3, leaving it
to the caller to drop the value later. When you do this, the value that
used to be behind the mutable reference is dropped immediately

另外，如果你不需要引用后面的旧值，你可以用一个你已经拥有的值来覆盖它3，让调用者以后再丢弃这个值。当你这样做的时候，原来在可变引用后面的值会被立即丢弃。


Finally, if you have two mutable references, you can swap their
values without owning either of them 4, since both references will
end up with a legal owned value for their owners to eventually free

最后，如果你有两个易变的引用，你可以在不拥有其中任何一个的情况下交换它们的值4，因为两个引用最后都会有一个合法拥有的值供它们的所有者最终释放。

### Interior Mutability

Some types provide interior mutability, meaning they allow you to
mutate a value through a shared reference. These types usually rely
on additional mechanisms (like atomic CPU instructions) or
invariants to provide safe mutability without relying on the
semantics of exclusive references. These normally fall into two
categories: those that let you get a mutable reference through a
shared reference, and those that let you replace a value given only a
shared reference.

有些类型提供内部可变性，这意味着它们允许你通过共享引用来变异一个值。这些类型通常依靠额外的机制（如原子CPU指令）或不变性来提供安全的可变性，而不依赖排他性引用的语义。这些通常分为两类：一类是让你通过共享引用获得一个可变的引用，另一类是让你替换一个只给定共享引用的值。

The first category consists of types like Mutex and RefCell, which
contain safety mechanisms to ensure that, for any value they give a
mutable reference to, only one mutable reference (and no shared
references) can exist at a time. Under the hood, these types (and
those like them) all rely on a type called UnsafeCell, whose name should
immediately make you hesitate to use it. We will cover UnsafeCell in
more detail in Chapter 9, but for now you should know that it is the
only correct way to mutate through a shared reference.

第一类由Mutex和RefCell等类型组成，它们包含安全机制，以确保对于它们给出的任何值的可变参考，一次只能存在一个可变参考（而没有共享参考）。在引擎盖下，这些类型（和那些类似的类型）都依赖于一个叫做UnsafeCell的类型，它的名字应该立刻让你对使用它感到犹豫。我们将在第9章详细介绍UnsafeCell，但现在你应该知道它是通过共享引用进行变异的唯一正确方法。

Other categories of types that provide interior mutability are those
that do not give out a mutable reference to the inner value but
instead just give you methods for manipulating that value in place.
The atomic integer types in std::sync::atomic and the std::cell::Cell type
fall into this category. You cannot get a reference directly to the usize
or i32 behind such a type, but you can read and replace its value at a
given point in time.

提供内部可变性的其他类别的类型是那些不给出内部值的可变性引用，而只是给你在原地操作该值的方法。std::sync::atomic中的原子整数类型和std::cell::Cell类型就属于这个类别。你不能直接获得对这种类型背后的usize或i32的引用，但你可以在一个给定的时间点上读取和替换它的值。

The Cell type in the standard library is an interesting
example of safe interior mutability through invariants. It is
not shareable across threads and never gives out a reference
to the value contained in the Cell. Instead, the methods all
either replace the value entirely or return a copy of the
contained value. Since no references can exist to the inner
value, it is always okay to move it. And since Cell isn’t
shareable across threads, the inner value will never be
concurrently mutated even though mutation happens
through a shared reference.

> 标准库中的Cell类型是一个有趣的例子，它通过不变量实现了安全的内部可变性。它不能跨线程共享，也不会给出对单元格中所包含的值的引用。相反，所有的方法要么完全替换该值，要么返回所含值的一个副本。因为没有对内部值的引用存在，所以移动它总是可以的。而且，由于Cell不能跨线程共享，即使变异是通过共享引用发生的，内部值也不会被并发变异。

### Lifetimes

If you’re reading this book, you’re probably already familiar with the
concept of lifetimes, likely through repeated notices from the
compiler about lifetime rules violations. That level of understanding
will serve you well for the majority of Rust code you will write, but as
we dive deeper into the more complex parts of Rust, you will need a
more rigorous mental model to work with.

如果你正在读这本书，你可能已经熟悉了生命期的概念，可能是通过编译器对生命期规则违反的反复通知。这种程度的理解对于你将要写的大多数Rust代码来说是很好的，但是当我们深入到Rust的更复杂的部分时，你将需要一个更严格的心理模型来工作。

Newer Rust developers are often taught to think of lifetimes as
corresponding to scopes: a lifetime begins when you take a reference
to some variable and ends when that variable is moved or goes out of
scope. That’s often correct, and usually useful, but the reality is a
little more complex. A lifetime is really a name for a region of code
that some reference must be valid for. While a lifetime will frequently
coincide with a scope, it does not have to, as we will see later in this
section.

新的Rust开发者经常被教导要把生命期看作是与作用域相对应的：当你获取某个变量的引用时，生命期就开始了，当该变量被移动或超出作用域时就结束了。这通常是正确的，而且通常很有用，但实际情况要复杂一些。生命期实际上是一个代码区域的名称，一些引用必须在该区域内有效。虽然生命期经常与作用域重合，但它并不是必须的，我们将在本节的后面看到。

#### Lifetimes and the Borrow Checker

At the heart of Rust lifetimes is the borrow checker. Whenever a
reference with some lifetime 'a is used, the borrow checker checks
that 'a is still alive. It does this by tracing the path back to where 'a
starts—where the reference was taken—from the point of use and
checking that there are no conflicting uses along that path. This
ensures that the reference still points to a value that it is safe to
access. This is similar to the high-level “data flow” mental model we
discussed earlier in the chapter; the compiler checks that the flow of
the reference we are accessing does not conflict with any other
parallel flows.

Rust lifetimes的核心是借用检查器。每当一个具有某种寿命的引用'a'被使用时，借用检查器就会检查'a'是否仍然活着。它通过追踪路径回到'a'开始的地方--引用被取走的地方--从使用点开始，并检查该路径上是否有冲突的使用。这确保了引用仍然指向一个可以安全访问的值。这类似于我们在本章前面讨论的高级 "数据流 "心理模型；编译器检查我们正在访问的引用的流不会与任何其他并行流相冲突。

Listing 1-8 shows a simple code example with lifetime annotations
for the reference to x.

清单1-8显示了一个简单的代码例子，其中有对x的引用的寿命注释。

```rust
// 1-8 shows a simple code example with lifetime annotations for the reference to x.
// listing 1-8: lifetimes dot not need to be contiguous.
let mut x = Box::new(42);
1: let r = &x; // 'a
if rand() > 0.5 {
  2: *x = 84;
}else{
  3: println!("{}", r); //'a
}
4:
```

Listing 1-8: Lifetimes do not need to be contiguous.

当我们获取对x的引用时，生命期从1开始。在第一个分支2中，我们立即尝试修改x，将其值改为84，这需要一个&mut x。它发现在获取引用和使用引用之间没有冲突，所以它接受了这段代码。如果你习惯于将生命周期视为作用域的话，这可能会让你感到惊讶，因为r在2的时候仍然在作用域中（在4的时候出了作用域）。但是借贷检查器足够聪明，它意识到如果这个分支被选中，以后就不会再使用r，因此x在这里被可变访问是没有问题的。或者，换句话说，在1处创建的生命期并没有延伸到这个分支：在2处之后没有来自r的流，因此没有冲突的流。然后，借贷检查器在3处的打印语句中发现了r的使用。它沿着路径回到1处，发现没有冲突的使用（2不在该路径上），所以它也接受这个使用。

The lifetime starts at 1 when we take a reference to x. In the first
branch 2, we then immediately try to modify x by changing its value
to 84, which requires a &mut x. The borrow checker takes out a mutable
reference to x and immediately checks its use. It finds no conflicting
uses between when the reference was taken and when it was used, so
it accepts the code. This may come as a surprise if you are used to
thinking about lifetimes as scopes, since r is still in scope at 2 (it
goes out of scope at 4). But the borrow checker is smart enough to
realize that r is never used later if this branch is taken, and therefore
it is fine for x to be mutably accessed here. Or, phrased differently,
the lifetime created at 1 does not extend into this branch: there is no
flow from r beyond 2, and therefore there are no conflicting flows.
The borrow checker then finds the use of r in the print statement at
3. It walks the path back to 1 and finds no conflicting uses (2 is
not on that path), so it accepts this use as well.

如果我们在清单1-8中的4处增加了对r的使用，代码就不能再编译了。a的生命周期将从1一直持续到4（r的最后一次使用），当借贷检查器检查我们对r的新使用时，它将发现2处有一个冲突的使用。

If we were to add another use of r at 4 in Listing 1-8, the code
would no longer compile. The lifetime 'a would then last from 1 all
the way until 4 (the last use of r), and when the borrow checker
checked our new use of r, it would discover a conflicting use at 2.

寿命可以变得相当复杂。在清单1-9中，你可以看到一个有漏洞的生命周期的例子，它在开始和最终结束的地方间歇性地无效。

Lifetimes can get quite convoluted. In Listing 1-9 you can see an
example of a lifetime that has holes, where it’s intermittently invalid
between where it starts and where it ultimately ends.

```rust
// listing 1-9: lifetimes can have holes.
let mut x = Box::new(42);
1: let mut z = &x; // 'a
for i in 0..100 {
  2: println!("{}",z); //'a
  3: x = Box::new(i);
  4: z = &x; //'a
}
println!("{}",z); // 'a
```

The lifetime starts at 1 when we take a reference to x. We then
move out of x at 3, which ends the lifetime 'a because it is no longer
valid. The borrow checker accepts this move by considering 'a ended
at 2, which leaves no conflicting flows from x at 3. Then, we restart
the lifetime by updating the reference in z 4. Regardless of whether
the code now loops back around to 2 or continues to the final print
statement, both of those uses now have a valid value to flow from,
and there are no conflicting flows, so the borrow checker accepts the
code!

当我们获取对x的引用时，生命期从1开始。然后我们在3处移出x，这结束了'a'的生命期，因为它不再有效。借贷检查器认为'a'在2处结束，这使得3处的x没有冲突的流量，从而接受了这一举动。 然后，我们通过更新z 4中的引用来重新启动生命周期。 无论代码现在是循环回到2处还是继续到最后的打印语句，这两个用途现在都有一个有效的值可以流出来，而且没有冲突的流量，所以借贷检查器接受了代码！

Again, this aligns perfectly with the data-flow model of memory we
discussed earlier. When x is moved, z stops existing. When we
reassign z later, we are creating an entirely new variable that exists
only from that point forward. It just so happens that that new
variable is also named z. With that model in mind, this example is
not weird.

同样，这与我们之前讨论的内存的数据流模型完全吻合。当x被移动时，z停止存在。当我们稍后重新分配z时，我们创建了一个全新的变量，这个变量只从这一点开始存在。碰巧的是，这个新变量也被命名为z。考虑到这个模型，这个例子并不奇怪。

The borrow checker is, and has to be, conservative. If it’s
unsure whether a borrow is valid, it rejects it, as the
consequences of allowing an invalid borrow could be
disastrous. The borrow checker keeps getting smarter, but
there are times when it needs help to understand why a
borrow is legal. This is part of why we have unsafe Rust

> 同样，这与我们之前讨论的内存的数据流模型完全吻合。当x被移动时，z停止存在。当我们稍后重新分配z时，我们创建了一个全新的变量，这个变量只从这一点开始存在。碰巧的是，这个新变量也被命名为z。考虑到这个模型，这个例子并不奇怪。



#### Gereric Lifetimes

Occasionally you need to store references within your own types.
Those references need to have a lifetime so that the borrow checker
can check their validity when they are used in the various methods
on that type. This is especially true if you want a method on your
type to return a reference that outlives the reference to self.

偶尔你需要在你自己的类型中存储引用。这些引用需要有一个生命周期，这样当它们被用于该类型的各种方法时，借贷检查器可以检查它们的有效性。如果你想让你的类型上的一个方法返回一个比对自己的引用更久远的引用，这一点尤其正确。

Rust lets you make a type definition generic over one or more
lifetimes, just as it allows you to make it generic over types. The Rust
Programming Language by Steve Klabnik and Carol Nichols (No
Starch Press, 2018) covers this topic in some detail, so I won’t
reiterate the basics here. But as you write more complex types of this
nature, there are two subtleties around the interaction between such
types and lifetimes that you should be aware of.

Rust允许你让一个类型定义在一个或多个生命期上通用，就像它允许你让它在类型上通用一样。Steve Klabnik和Carol Nichols的《Rust编程语言》（No Starch Press，2018）对这一主题做了一些详细介绍，所以我不会在这里重申基础知识。但是，当你编写这种性质的更复杂的类型时，围绕这种类型和生命期之间的互动，有两个微妙的问题，你应该注意。

First, if your type also implements Drop, then dropping your type
counts as a use of any lifetime or type your type is generic over.
Essentially, when an instance of your type is dropped, the borrow
checker will check that it’s still legal to use any of your type’s generic
lifetimes before dropping it. This is necessary in case your drop code
does use any of those references. If your type does not implement
Drop, dropping the type does not count as a use, and users are free to
ignore any references stored in your type as long as they do not use it
anymore, like we saw in Listing 1-7. We’ll talk more about these rules
around dropping in Chapter 9.

首先，如果你的类型也实现了Drop，那么丢弃你的类型也算作是对你的类型的泛型寿命或类型的使用。基本上，当你的类型的一个实例被丢弃时，借贷检查器将检查在丢弃它之前使用你的类型的任何泛型寿命是否仍然合法。这是必要的，以防你的丢弃代码确实使用了任何这些引用。如果你的类型没有实现 Drop，丢弃这个类型就不算是使用，用户只要不再使用你的类型，就可以自由地忽略存储在你的类型中的任何引用，就像我们在清单 1-7 中看到的那样。我们将在第9章中更多地讨论这些关于放弃的规则。

Second, while a type can be generic over multiple lifetimes, making
it so often only serves to unnecessarily complicate your type
signature. Usually, a type being generic over a single lifetime is fine,
and the compiler will use the shorter of the lifetimes for any
references inserted into your type as that one lifetime. You should
only really use multiple generic lifetime parameters if you have a
type that contains multiple references, and its methods return
references that should be tied to the lifetime of only one of those
references.

第二，虽然一个类型可以在多个生命周期内通用，但经常这样做只会让你的类型签名变得不必要的复杂。通常情况下，一个类型在一个生命周期内是通用的就可以了，编译器会将插入到你的类型中的任何引用的生命周期中较短的一个作为这个生命周期。只有当你有一个包含多个引用的类型，而它的方法返回的引用应该只与其中一个引用的寿命挂钩时，你才应该真正使用多个泛型寿命参数。


Consider the type in Listing 1-10, which gives you an iterator over
parts of a string separated by a particular other string.

考虑一下清单1-10中的类型，它为你提供了一个由特定的其他字符串分隔的字符串部分的迭代器。

```rust
// Listing 1-10: A type that needs to be generic over multiple lifetimes
struct StrSplit<'s, 'p> {
  delimiter: &'p str,
  document: &'s str,
}
impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
  type Item = &'s str;
  fn next(&self) -> Option<Self::Item> {
    todo!()
  }
}

fn str_before(s: &str,c: char) -> Option<&str> {
  StrSplit{ document: s, delimiter: &c.to_string()}.next()
}
```

Listing 1-10: A type that needs to be generic over multiple
lifetimes

When you construct this type, you have to give the delimiter and
document to search, both of which are references to string values. When
you ask for the next string, you get a reference into the document.
Consider what would happen if you used a single lifetime in this type.
The values yielded by the iterator would be tied to the lifetime of the
document and the delimiter. This would make str_before impossible to
write: the return type would have a lifetime associated with a
variable local to the function—the String produced by to_string—and
the borrow checker would reject the code.

当你构建这个类型时，你必须给出分隔符和要搜索的文档，这两个都是对字符串值的引用。当你要求下一个字符串时，你会得到一个对文档的引用。考虑一下如果你在这个类型中使用一个单一的生命周期会发生什么。迭代器产生的值将与文档的生命周期和分隔符相联系。这将使str_before无法编写：返回类型将有一个与函数本地变量相关的生命周期--to_string产生的String--借贷检查器将拒绝该代码。

#### Lifetime Variance

Variance is a concept that programmers are often exposed to but
rarely know the name of because it’s mostly invisible. At a glance,
variance describes what types are subtypes of other types and when a
subtype can be used in place of a supertype (and vice versa). Broadly
speaking, a type A is a subtype of another type B if A is at least as useful
as B. Variance is the reason why, in Java, you can pass a Turtle to a
function that accepts an Animal if Turtle is a subtype of Animal, or why, in
Rust, you can pass a &'static str to a function that accepts a &'a str.

"差异 "是程序员经常接触到的一个概念，但很少知道它的名字，因为它大多是看不见的。简而言之，差异性描述了哪些类型是其他类型的子类型，以及何时可以用子类型来代替超类型（反之亦然）。广义上讲，如果一个类型A至少和B一样有用，那么它就是另一个类型B的子类型。 Variance就是为什么在Java中，如果Turtle是Animal的子类型，你可以把Turtle传给接受Animal的函数，或者在Rust中，你可以把&'static str传给接受&'a str的函数。

While variance usually hides out of sight, it comes up often enough
that we need to have a working knowledge of it. Turtle is a subtype of
Animal because a Turtle is more “useful” than some unspecified Animal—a
Turtle can do anything an Animal can do, and likely more. Similarly,
'static is a subtype of 'a because a 'static lives at least as long as any 'a
and so is more useful. Or, more generally, if 'b: 'a ('b outlives 'a), then
'b is a subtype of 'a. This is obviously not the formal definition, but it
gets close enough to be of practical use.

虽然变异通常隐藏在视线之外，但它经常出现，我们需要对它有一个工作上的了解。乌龟是动物的一个亚类型，因为乌龟比某些未指定的动物更 "有用"--乌龟可以做任何动物能做的事情，而且可能更多。同样，"静态 "是 "a "的一个子类型，因为 "静态 "的寿命至少与任何 "a "一样长，所以更有用。或者，更一般地说，如果'b：'a（'b比'a长寿），那么'b就是'a的一个子类型。这显然不是正式的定义，但它已经足够接近实际用途了。

All types have a variance, which defines what other similar types
can be used in that type’s place. There are three kinds of variance:
covariant, invariant, and contravariant. A type is covariant if you can
just use a subtype in place of the type. For example, if a variable is of
type &'a T, you can provide a value of type &'static T to it, because &'a T
is covariant in 'a. &'a T is also covariant in T, so you can pass a
&Vec<&'static str> to a function that takes &Vec<&'a str>.

所有类型都有一个方差，它定义了哪些其他类似的类型可以用于该类型的位置。有三种方差：共变、不变、和不变型。如果你可以只使用一个子类型来代替该类型，那么该类型就是协变的。例如，如果一个变量是&'a T类型的，你可以给它提供一个&'static T类型的值，因为&'a T在'a中是协变的。&'a T在T中也是协变的，所以你可以把一个&Vec<&'static str>传递给一个接受&Vec<&'a str>的函数。

Some types are invariant, which means that you must provide
exactly the given type. &mut T is an example of this—if a function takes
a &mut Vec<&'a str>, you cannot pass it a &mut Vec<&'static str>. That is, &mut T is invariant in T. If you could, the function could put a short-lived string inside the Vec, which the caller would then continue using,
thinking that it were a Vec<&'static str> and thus that the contained
string were 'static! Any type that provides mutability is generally
invariant for the same reason—for example, Cell<T> is invariant in T.
有些类型是不变的，这意味着你必须准确地提供给定的类型。&mut T就是这样一个例子--如果一个函数接受一个&mut Vec<&'a str>，你不能把一个&mut Vec<&'static str>传递给它。也就是说，&mut T在T中是不变的。如果你可以，函数可以在Vec中放入一个短暂的字符串，然后调用者会继续使用它，认为它是一个Vec<&'static str>，从而认为包含的字符串是 "静态的！"。任何提供可变性的类型一般都是不变的，原因也是如此--例如，Cell<T>在T中是不变的。

The last category, contravariance, comes up for function
arguments. Function types are more useful if they’re okay with their
arguments being less useful. This is clearer if you contrast the
variance of the argument types on their own with their variance
when used as function arguments:

最后一类，即禁忌，出现在函数参数上。如果函数类型可以接受其参数不那么有用，那么它们就会更有用。如果你将参数类型本身的方差与它们作为函数参数时的方差进行对比，这一点就更清楚了：

```rust
let x: &'static str; // mote useful, live longer
let x: &'a str; // less useful, lives shorter

fn take_func1(&'static str) // stricter, so less useful
fn take_func2(&'a str) // less strict, more useful
```

This flipped relationship indicates that Fn(T) is contravariant in T.

这种翻转的关系表明，Fn(T)在T中是禁变量。

So why do you need to learn about variance when it comes to
lifetimes? Variance becomes relevant when you consider how generic
lifetime parameters interact with the borrow checker. Consider a
type like the one shown in Listing 1-11, which uses multiple lifetimes
in a single field.

那么，当涉及到寿命时，你为什么需要学习方差呢？当你考虑通用寿命参数如何与借贷检查器交互时，方差就变得相关了。考虑像清单1-11中所示的类型，它在一个字段中使用了多个寿命。

```rust
// listing1-11: A type that needs to be generic over multiple lifetimes
struct MutStr<'a, 'b> {
  s: &'a mut &'b str
}
let mut s = "Hello";
1: &MutStr {s: &mut s}.s = "world";
println!("{}", s);
```
Listing 1-11: A type that needs to be generic over multiple
lifetimes

At first glance, using two lifetimes here seems unnecessary—we
have no methods that need to differentiate between a borrow of
different parts of the structure, as we did with StrSplit in Listing 1-10.
But if you replace the two lifetimes here with a single 'a, the code no
longer compiles! And it’s all because of variance.

乍一看，在这里使用两个生命期似乎没有必要--我们没有需要区分结构中不同部分的借贷的方法，就像我们在清单1-10中的StrSplit那样。但是如果你用一个'a'来代替这里的两个生命期，代码就不再能编译了！这都是因为变异。而这一切都是因为变异。

Note: The syntax at 1 may seem alien. It’s equivalent to defining a
variable x holding a MutStr and then writing *x.s = "world",
except that there’s no variable and so the MutStr is dropped
immediately


> 1处的语法可能看起来很陌生。它相当于定义了一个持有MutStr的变量x，然后写*x.s = "world"，只是没有变量，所以MutStr被立即删除。

At 1, the compiler must determine what lifetime the lifetime
parameter(s) should be set to. If there are two lifetimes, 'a is set to
the to-be-determined lifetime of the borrow of s, and 'b is set to 'static
since that’s the lifetime of the provided string "hello". If there is just
one lifetime 'a, the compiler infers that that lifetime must be 'static.

在1处，编译器必须确定寿命参数应该被设置为什么寿命。如果有两个寿命，'a'被设置为有待确定的s的借贷寿命，'b'被设置为'静态'，因为那是提供的字符串 "hello "的寿命。如果只有一个生命期'a'，编译器会推断该生命期必须是'静态'。

When we later attempt to access the string reference s through a
shared reference to print it, the compiler tries to shorten the mutable
borrow of s used by MutStr to allow the shared borrow of s.

当我们以后试图通过共享引用访问字符串引用s来打印它时，编译器会尝试缩短MutStr使用的s的可变借用，以允许s的共享借用。

In the two-lifetime case, 'a simply ends just before the println, and
'b stays the same. In the single-lifetime case, on the other hand, we
run into issues. The compiler wants to shorten the borrow of s, but to
do so, it would also have to shorten the borrow of the str. While
&'static str can in general be shortened to any &'a str (&'a T is covariant
in 'a), here it’s behind a &mut T, which is invariant in T. Invariance
requires that the relevant type is never replaced with a sub- or
supertype, so the compiler’s attempt to shorten the borrow fails, and
it reports that the list is still mutably borrowed. Ouch!

在双生命周期的情况下，'a'只是在println之前结束，'b'保持不变。另一方面，在单时限的情况下，我们遇到了问题。编译器想缩短s的借用，但要做到这一点，它也必须缩短str的借用。虽然&'static str一般来说可以缩短为任何&'a str（&'a T在'a中是共变的），但这里它在&mut T后面，而&mut T在T中是不变量的。不变量要求相关类型永远不会被子类型或超类型取代，所以编译器缩短借用的尝试失败了，它报告说列表仍然是可变的借用。哎哟！

Because of the reduced flexibility imposed by invariance, you want
to ensure that your types remain covariant (or contravariant where
appropriate) over as many of their generic parameters as possible. If
that requires introducing additional lifetime arguments, you need to
carefully weigh the cognitive cost of adding another parameter
against the ergonomic cost of invariance.

由于不变性带来的灵活性的降低，你想确保你的类型在尽可能多的通用参数上保持协变性（或在适当的情况下保持协变性）。如果这需要引入额外的寿命参数，你需要仔细权衡增加另一个参数的认知成本和不变性的人机工程成本。

## Summary

The aim of this chapter has been to establish a solid, shared
foundation that we can build on in the chapters to come. By now, I
hope you feel that you have a firm grasp on Rust’s memory and
ownership model, and that those errors you may have gotten from the borrow checker seem less mysterious. You might have known bits
and pieces of what we covered here already, but hopefully the
chapter has given you a more holistic image of how it all fits together.
In the next chapter, we will do something similar for types. We’ll go
over how types are represented in memory, see how generics and
traits produce running code, and take a look at some of the special
type and trait constructs Rust offers for more advanced use cases.

本章的目的是建立一个坚实的共享基础，以便在接下来的章节中继续构建。现在，我希望您已经对Rust的内存和所有权模型有了坚实的理解，并且那些可能由借用检查器引发的错误看起来不再神秘。您可能已经了解了本章中讨论的某些部分，但希望本章能够为您提供一个更全面的图像，以便您了解它们是如何相互关联的。在下一章中，我们将对类型进行类似的讲解。我们将介绍类型在内存中的表示方式，了解泛型和特性如何产生运行代码，并查看一些Rust为更高级用例提供的特殊类型和特性构造。