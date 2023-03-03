# Chapter 2: Types

Now that the fundamentals are out of the way, we’ll look at Rust’s type system. We’ll skip past the basics covered in The Rust Programming Language and instead dive headfirst into how different types are laid out in memory, the ins and outs of traits and trait bounds, existential types, and the rules for using types across crate boundaries.

## Types in Memory

Every Rust value has a type. Types serve many purposes in Rust, as
we’ll see in this chapter, but one of their most fundamental roles is to
tell you how to interpret bits of memory. For example, the sequence
of bits 0b10111101 (written in hexadecimal notation as 0xBD) does not
mean anything in and of itself until you assign it a type. When
interpreted under the type u8, that sequence of bits is the number
189. When interpreted under the type i8, it is –67. When you define
your own types, it’s the compiler’s job to determine where each part
of the defined type goes in the in-memory representation for that
type. Where does each field of your struct appear in the sequence of
bits? Where is the discriminant for your enum stored? It’s important
to understand how this process works as you begin to write more
advanced Rust code, because these details affect both the correctness
and the performance of your code.

### Alignment

Before we talk about how a type’s in-memory representation is
determined, we first need to discuss the notion of alignment, which
dictates where the bytes for a type can be stored. Once a type’s
representation has been determined, you might think you can take
any arbitrary memory location and interpret the bytes stored there as
that type. While that is true in a theoretical sense, in practice the
hardware also constrains where a given type can be placed. The most
obvious example of this is that pointers point to bytes, not bits. If you
placed a value of type T starting at bit 4 of your computer’s memory,
you would have no way to refer to its location; you can create a
pointer pointing only to byte 0 or byte 1 (bit 8). For this reason, all
values, no matter their type, must start at a byte boundary. We say
that all values must be at least byte-aligned—they must be placed at
an address that is a multiple of 8 bits.

Some values have more stringent alignment rules than just being
byte-aligned. In the CPU and the memory system, memory is often
accessed in blocks larger than a single byte. For example, on a 64-bit
CPU, most values are accessed in chunks of 8 bytes (64 bits), with
each operation starting at an 8-byte-aligned address. This is referred
to as the CPU’s word size. The CPU then uses some cleverness to
handle reading and writing smaller values, or values that span the
boundaries of these chunks.

Where possible, you want to ensure that the hardware can operate
in its “native” alignment. To see why, consider what happens if you
try to read an i64 that starts in the middle of an 8-byte block (that is,
the pointer to it is not 8-byte-aligned). The hardware will have to do
two reads—one from the second half of the first block to get to the
start of the i64, and one from the first half of the second block to read
the rest of the i64—and then splice the results together. This is not
very efficient. Since the operation is spread across multiple accesses
to the underlying memory, you may also end up with strange results
if the memory you’re reading from is concurrently written to by a
different thread. You might read the first 4 bytes before the other
thread’s write has happened and the second 4 bytes after, resulting in
a corrupted value

Operations on data that is not aligned are referred to as
misaligned accesses and can lead to poor performance and bad
concurrency problems. For this reason, many CPU operations
require, or strongly prefer, that their arguments are naturally
aligned. A naturally aligned value is one whose alignment matches
its size. So, for example, for an 8-byte load, the provided address
would need to be 8-byte-aligned.

Since aligned accesses are generally faster and provide stronger
consistency semantics, the compiler tries to take advantage of them
where possible. It does this by giving every type an alignment that’s
computed based on the types that it contains. Built-in values are
usually aligned to their size, so a u8 is byte-aligned, a u16 is 2-bytealigned, a u32 is 4-byte-aligned, and a u64 is 8-byte-aligned. Complex
types—types that contain other types—are typically assigned the
largest alignment of any type they contain. For example, a type that
contains a u8, a u16, and a u32 will be 4-byte-aligned because of the u32.

### Layout

Now that you know about alignment, we can explore how the
compiler decides on the in-memory representation, known as the
layout, of a type. By default, as you’ll see shortly, the Rust compiler
gives very few guarantees about how it lays out types, which makes
for a poor starting point for understanding the underlying principles.
Luckily, Rust provides a repr attribute you can add to your type
definitions to request a particular in-memory representation for that
type. The most common one you will see, if you see one at all, is
repr(C). As the name suggests, it lays out the type in a way that is
compatible with how a C or C++ compiler would lay out the same
type. This is helpful when writing Rust code that interfaces with
other languages using the foreign-function interface, which we’ll talk
about in Chapter 11, as Rust will generate a layout that matches the
expectations of the other language’s compiler. Since the C layout is
predictable and not subject to change, repr(C) is also useful in unsafe
contexts if you’re working with raw pointers into the type, or if you
need to cast between two different types that you know have the
same fields. And, of course, it is perfect for taking our first steps into
layout algorithms.

> Note: used only on types with a single field and which guarantees
that the layout of the outer type is exactly the same as that of
the inner type. This comes in handy in combination with the
“newtype” pattern, where you may want to operate on the
in-memory representations of some struct A and struct NewA(A)
as if they were the same. Without repr(transparent), the Rust
compiler does not guarantee that they will have the same
layout.

So, let’s look how the compiler would lay out a particular type with
repr(C): the Foo type in Listing 2-1. How do you think the compiler
would lay this out in memory?

```rust
#[repr(C)]
struct Foo {
 tiny: bool,
 normal: u32,
 small: u8,
 long: u64,
 short: u16,
}
```

Listing 2-1: Alignment affects layout.

First the compiler sees the field tiny, whose logical size is 1 bit (true
or false). But since the CPU and memory operate in terms of bytes,
tiny is given 1 byte in the in-memory representation. Next, normal is a
4-byte type, so we want it to be 4-byte-aligned. But even if Foo is
aligned, the 1 byte we allocated to tiny is going to make normal miss its
alignment. To rectify this, the compiler inserts 3 bytes of padding—
bytes with an indeterminate value that are ignored in user code—into
the in-memory representation between tiny and normal. No values go
into the padding, but it does take up space.

For the next field, small, alignment is simple: it’s a 1-byte value, and
the current byte offset into the struct is 1 + 3 + 4 = 8. This is already
byte-aligned, so small can go immediately after normal. With long we
have a problem again, though. We are now 1 + 3 + 4 + 1 = 9 bytes
into Foo. If Foo is aligned, then long is not 8-byte-aligned the way we
want it to be, so we must insert another 7 bytes of padding to make
long aligned again. This also conveniently ensures the 2-byte
alignment we need for the last field, short, bringing the total to 26
bytes. Now that we’ve gone through all the fields, we also need to
determine the alignment of Foo itself. The rule here is to use the
largest alignment of any of Foo’s fields, which will be 8 bytes because
of long. So, to ensure that Foo remains aligned if placed in, say, an
array, the compiler then adds a final 6 bytes of padding to make Foo’s
size a multiple of its alignment at 32 bytes.

Now we are ready to shed the C legacy and consider what would
happen to the layout if we did not use repr(C) in Listing 2-1. One of the
primary limitations of the C representation is that it requires that we
place all fields in the same order that they appear in the original
struct definition. The default Rust representation repr(Rust) removes
that limitation, along with a couple of other lesser restrictions, such
as deterministic field ordering for types that happen to have the
same fields. That is, even two different types that share all the same
fields, of the same type, in the same order, are not guaranteed to be
laid out the same when using the default Rust layout!


Since we’re now allowed to reorder the fields, we can place them in
decreasing order of size. This means we no longer need the padding
between Foo’s fields; the fields themselves are used to achieve the
necessary alignment! Foo is now just the size of its fields: only 16
bytes. This is one of the reasons why Rust by default does not give
many guarantees about how a type is laid out in memory: by giving
the compiler more leeway to rearrange things, we can produce more
efficient code.

It turns out there’s also a third way to lay out a type, and that is to
tell the compiler that we do not want any padding between our fields.
In doing so, we’re saying that we are willing to take the performance
hit of using misaligned accesses. The most common use case for this
is when the impact of every additional byte of memory can be felt,
such as if you have a lot of instances of the type, if you have very
limited memory, or if you’re sending the in-memory representation
over a lower-bandwidth medium like a network connection. To opt in
to this behavior, you can annotate your type with #[repr(packed)]. Keep
in mind that this may lead to much slower code, and in extreme
cases, this can cause your program to crash if you try to perform
operations that the CPU supports only on aligned arguments.


Sometimes, you want to give a particular field or type a larger
alignment than it technically requires. You can do that using the
attribute #[repr(align(n))]. A common use case for this is to ensure that
different values stored contiguously in memory (like in an array) end
up in different cache lines on the CPU. That way, you avoid false
sharing, which can cause huge performance degradations in
concurrent programs. False sharing occurs when two different CPUs
access different values that happen to share a cache line; while they
can theoretically operate in parallel, they both end up contending to
update the same single entry in the cache. We’ll talk about
concurrency in much greater detail in Chapter 10.

### Complex Types

You might be curious about how the compiler represents other Rust
types in memory. Here’s a quick reference:
- Tuple Represented like a struct with fields of the same type as
the tuple values in the same order.
- Array Represented as a contiguous sequence of the contained
type with no padding between the elements.
- Union Layout is chosen independently for each variant.
Alignment is the maximum across all the variants.
- Enumeration Same as union, but with one additional hidden
shared field that stores the enum variant discriminant. The
discriminant is the value the code uses to determine which of the
enum variants a given value holds. The size of the discriminant
field depends on the number of variants.

### Dynamically Sized Types and wide Pointers

You may have come across the marker trait Sized in various odd
corners of the Rust documentation and in error messages. Usually, it
comes up because the compiler wants you to provide a type that is
Sized, but you (apparently) did not. Most types in Rust implement
Sized automatically—that is, they have a size that’s known at compile
time—but two common types do not: trait objects and slices. If you
have, for example, a dyn Iterator or a [u8], those do not have a welldefined size. Their size depends on some information that is known
only when the program runs and not at compile time, which is why
they are called dynamically sized types (DSTs). Nobody knows
ahead of time whether the dyn Iterator your function received is this
200-byte struct or that 8-byte struct. This presents a problem: often
the compiler must know the size of something in order to produce
valid code, such as how much space to allocate to a tuple of type (i32,
dyn Iterator, [u8], i32) or what offset to use if your code tries to access
the fourth field. But if the type isn’t Sized, that information isn’t
available.

The compiler requires types to be Sized nearly everywhere. Struct
fields, function arguments, return values, variable types, and array
types must all be Sized. This restriction is so common that every single
type bound you write includes T: Sized unless you explicitly opt out of
it with T: ?Sized (the ? means “may not be”). But this is pretty
unhelpful if you have a DST and want to do something with it, like if
you really want your function to accept a trait object or a slice as an
argument.

lace unsized types behind a wide pointer (also known as a fat
pointer). A wide pointer is just like a normal pointer, but it includes
an extra word-sized field that gives the additional information about
that pointer that the compiler needs to generate reasonable code for
working with the pointer. When you take a reference to a DST, the
compiler automatically constructs a wide pointer for you. For a slice,
the extra information is simply the length of the slice. For a trait
object—well, we’ll get to that later. And crucially, that wide pointer is
Sized. Specifically, it is twice the size of a usize (the size of a word on
the target platform): one usize for holding the pointer, and one usize
for holding the extra information needed to “complete” the type.


> Note: Box and Arc also support storing wide pointers, which is why
they both support T: ?Sized.

### Traits and Trait Bounds

Traits are a key piece of Rust’s type system—they are the glue that
allows types to interoperate even though they don’t know about each
other at the time they are defined. The Rust Programming
Language does a great job of covering how to define and use traits,
so I won’t go over that here. Instead, we’re going to take a look at
some of the more technical aspects of traits: how they’re
implemented, restrictions you have to adhere to, and some more
esoteric uses of traits.

#### Compilation and Dispatch

By now, you’ve probably written a decent amount of generic code in
Rust. You’ve used generic type parameters on types and methods,
and maybe even a few trait bounds here and there. But have you ever
wondered what actually happens to generic code when you compile
it, or what happens when you call a trait method on a dyn Trait?

When you write a type or function that is generic over T, you’re
really telling the compiler to make a copy of that type or function for
each type T. When you construct a Vec<i32> or a HashMap<String, bool>, the
compiler essentially copy-pastes the generic type and all its
implementation blocks and replaces all instances of each generic
parameter with the concrete type you provided. It makes a full copy
of the Vec type with every T replaced with i32, and a full copy of the
HashMap type with every K replaced with String and every V with bool.

> Note: In reality, the compiler does not actually do a full copy-paste.
It copies only parts of the code that you use, so if you never
call find on a Vec<i32>, the code for find won’t be copied and
compiled.

The same thing applies to generic functions. Consider the code in
Listing 2-2, which shows a generic method.

```rust
impl String {
 pub fn contains(&self, p: impl Pattern) -> bool {
 p.is_contained_in(self)
 }
}
```

Listing 2-2: A generic method using static dispatch

A copy of this method is made for every distinct pattern type
(recall that impl Trait is shorthand for <T: Trait>). We need a different
copy of the function body for each impl Pattern type because we need to
know the address of the is_contained_in function to call it. The CPU
needs to be told where to jump to and continue execution. For any
given pattern, the compiler knows that that address is the address of
the place where that pattern type implements that trait method. But
there is no one address we could use for any type, so we need to have
one copy for each type, each with its own address to jump to. This is
referred to as static dispatch, since for any given copy of the method,
the address we are “dispatching to” is known statically.


> Note: You may have noticed that the word “static” is a little
overloaded in this context. Static is generally used to refer to
anything that is known at compile time, or can be treated as
though it were, since it can then be written into static
memory, as we discussed in Chapter 1.

This process of going from a generic type to many non-generic
types is called monomorphization, and it’s part of the reason generic
Rust code usually performs just as well as non-generic code. By the
time the compiler starts optimizing your code, it’s as if no generics
were there at all! Each instance is optimized separately and with all
of the types known. As a result, the code is just as efficient as if the
is_contained_in method of the pattern that is passed in were called
directly without any traits present. The compiler has full knowledge
of the types involved and can even inline the implementation of
is_contained_in if it wishes

Monomorphization also comes at a cost: all those instantiations of
your type need to be compiled separately, which can increase
compile time if the compiler cannot optimize them away. Each
monomorphized function also results in its own chunk of machine
code, which can make your program larger. And because instructions
aren’t shared between different instantiations of a generic type’s
methods, the CPU’s instruction cache is also less effective as it now
needs to hold multiple copies of effectively the same instructions.

> NON-GENERIC INNER FUNCTIONS
> 
> Often, much of the code in a generic method is not type-dependent. Consider, for
example, the implementation of HashMap::insert. The code to compute the hash of the
supplied key depends on the key type of the map, but the code to walk the buckets of
the map to find the insertion point may not. In cases like this, it would be more efficient
to share the generated machine code for the non-generic parts of the method across
monomorphizations, and only generate distinct copies where this is actually needed.
>
> One pattern you can use for cases like this is to declare a non-generic helper
function inside the generic method that performs the shared operations. This leaves
only the type-dependent code for the compiler to copy-paste for you while allowing the
helper function to be shared.
> 
> Making the function an inner function comes with the added benefit that you do not
pollute your module with a single-purpose function. You can instead declare such a
helper function outside the method instead; just be careful that you don’t make it a
method under a generic impl block, as then it will still be monomorphized.


The alternative to static dispatch is dynamic dispatch, which
enables code to call a trait method on a generic type without knowing
what that type is. I said earlier that the reason we needed multiple
instances of the method in Listing 2-2 was that otherwise your
program wouldn’t know what address to jump to in order to call the
trait method is_contained_in on the given pattern. Well, with dynamic
dispatch, the caller simply tells you. If you replace impl Pattern with &dyn
Pattern, you tell the caller that they must give two pieces of
information for this argument: the address of the pattern and the
address of the is_contained_in method. In practice, the caller gives us a
pointer to a chunk of memory called a virtual method table, or
vtable, that holds the address of the implementation of all the trait’s
methods for the type in question, one of which is is_contained_in. When
the code inside the method wants to call a trait method on the
provided pattern, it looks up the address of that pattern’s
implementation of is_contained_in in the vtable and then calls the
function at that address. This allows us to use the same function
body regardless of what type the caller wants to use.

> Note: Every vtable also contains information about the concrete
type’s layout and alignment since that information is always
needed to work with a type. If you want an example of what
an explicit vtable looks like, take a look at the
std::task::RawWakerVTable type.

You’ll notice that when we opted in to dynamic dispatch using the
dyn keyword, we had to place an & in front of it. The reason is that we
no longer know at compile time the size of the pattern type that the
caller passes in, so we don’t know how much space to set aside for it.
In other words, dyn Trait is !Sized, where the ! means not. To make it
Sized so we can take it as an argument, we place it behind a pointer
(which we know the size of). Since we also need to pass along the
table of method addresses, this pointer becomes a wide pointer,
where the extra word holds the pointer to the vtable. You can use any
type that is able to hold a wide pointer for dynamic dispatch, such as
&mut, Box, and Arc. Listing 2-3 shows the dynamic dispatch equivalent of
Listing 2-2.

```rust
impl String {
 pub fn contains(&self, p: &dyn Pattern) -> bool {
 p.is_contained_in(&*self)
 }
}
```
Listing 2-3: A generic method using dynamic dispatch

The combination of a type that implements a trait and its vtable is
known as a trait object. Most traits can be turned into trait objects,
but not all. For example, the Clone trait, whose clone method returns
Self, cannot be turned into a trait object. If we accept a dyn Clone trait
object and then call clone on it, the compiler won’t know what type to
return. Or, consider the Extend trait from the standard library, which
has a method extend that is generic over the type of the provided
iterator (so there may be many instances of it). If you were to call a
method that took a dyn Extend, there would be no single address for
extend to place in the trait object’s vtable; there would have to be one
entry for every type extend might ever be called with. These are
examples of traits that are not object-safe and therefore may not be
turned into trait objects. To be object-safe, none of a trait’s methods
can be generic or use the Self type. Furthermore, the trait cannot
have any static methods (that is, methods whose first argument does
not dereference to Self), since it would be impossible to know which
instance of the method to call. It is not clear, for example, what code
FromIterator::from_iter(&[0]) should execute.

When reading about trait objects, you may see mentions of the
trait bound Self: Sized. Such a bound implies that Self is not being
used through a trait object (since it would then be !Sized). You can
place that bound on a trait to require that the trait never use dynamic
dispatch, or you can place it on a specific method to make that
method unavailable when the trait is accessed through a trait object.
Methods with a where Self: Sized bound are exempted when checking if
a trait is object-safe.

Dynamic dispatch cuts compile times, since it’s no longer
necessary to compile multiple copies of types and methods, and it
can improve the efficiency of your CPU instruction cache. However,
it also prevents the compiler from optimizing for the specific types
that are used. With dynamic dispatch, all the compiler can do for find
in Listing 2-2 is insert a call to the function through the vtable—it
can no longer perform any additional optimizations as it does not
know what code will sit on the other side of that function call.
Furthermore, every method call on a trait object requires a lookup in
the vtable, which adds a small amount of overhead over calling the
method directly.

When you’re given the choice between static and dynamic
dispatch, there is rarely a clear-cut right answer. Broadly speaking,
though, you’ll want to use static dispatch in your libraries and
dynamic dispatch in your binaries. In a library, you want to allow
your users to decide what kind of dispatch is best for them, since you
don’t know what their needs are. If you use dynamic dispatch, they’re
forced to do the same, whereas if you use static dispatch, they can
choose whether to use dynamic dispatch or not. In a binary, on the
other hand, you’re writing the final code, so there are no needs to
consider except those of the code you are writing. Dynamic dispatch
often allows you to write cleaner code that leaves out generic
parameters and will compile more quickly, all at a (usually) marginal
performance cost, so it’s usually the better choice for binaries.

### Generic Traits

Rust traits can be generic in one of two ways: with generic type
parameters like trait Foo<T> or with associated types like trait Foo { type
Bar; }. The difference between these is not immediately apparent, but
luckily the rule of thumb is quite simple: use an associated type if you
expect only one implementation of the trait for a given type, and use
a generic type parameter otherwise.

The rationale for this is that associated types are often significantly
easier to work with, but will not allow multiple implementations. So,
more simply put, the advice is really just to use associated types
whenever you can.

With a generic trait, users must always specify all the generic
parameters and repeat any bounds on those parameters. This can
quickly get messy and hard to maintain. If you add a generic
parameter to a trait, all users of that trait must also be updated to
reflect the change. And since multiple implementations of a trait may
exist for a given type, the compiler may have a hard time deciding
which instance of the trait you meant to use, leading to awful
disambiguating function calls like FromIterator::<u32>::from_iter. But the
upside is that you can implement the trait multiple times for the
same type—for example, you can implement PartialEq against multiple
right-hand side types for your type, or you can implement both
FromIterator<T> and FromIterator<&T> where T: Clone, precisely because of the
flexibility that generic traits provide.

With associated types, on the other hand, the compiler needs to
know only the type that implements the trait, and all the associated
types follow (since there is only one implementation). This means
the bounds can all live in the trait itself and do not need to be
repeated on use. In turn, this allows the trait to add further
associated types without affecting its users. And because the type
dictates all the associated types of the trait, you never have to
disambiguate with the unified function calling syntax shown in the
previous paragraph. However, you cannot implement Deref against
multiple Target types, nor can you implement Iterator with multiple
different Item types.


### Coherence and the Orphan Rule

Rust has some fairly strict rules about where you can implement
traits and what types you can implement them on. These rules exist
to preserve the coherence property: for any given type and method,
there is only ever one correct choice for which implementation of the
method to use for that type. To see why this is important, consider
what would happen if I could write my own implementation of the
Display trait for the bool type from the standard library. Now, for any
code that tries to print a bool value and includes my crate, the
compiler won’t know whether to pick the implementation I wrote or
the one from the standard library. Neither choice is correct or better
than the other, and the compiler obviously cannot choose randomly.
The same issue occurs if the standard library is not involved at all,
but we instead have two crates that depend on each other, and they
both implement a trait for some shared type. The coherence property
ensures that the compiler never ends up in these situations and
never has to make these choices: there will always be exactly one
obvious choice.

A facile way to uphold coherence would be to ensure only the crate
that defines a trait can write implementations for that trait; if no one
else can implement the trait, then there can be no conflicting
implementations elsewhere. However, this is too restrictive in
practice and would essentially make traits useless, as there would be
no way to implement traits like std::fmt::Debug and serde::Serialize for
your own types, unless you got your own type included into the
defining crate. The opposite extreme, saying that you can implement
traits for only your own types, solves that problem but introduces
another: a crate that defines a trait now cannot provide
implementations of that trait for types in the standard library or in
other popular crates! Ideally, we would like to find some set of rules
that balances the desire for downstream crates to implement
upstream traits for their own types against the desire for upstream
crates to be able to add implementations of their own traits without
breaking downstream code

> Upstream refers to something your code depends on, and
downstream refers to something that depends on your code.
Often, these terms are used in the direct sense of crate
dependencies, but they can also be used to refer to
authoritative forks of a codebase—if you do a fork of the Rust
compiler, the official Rust compiler is your “upstream.”

In Rust, the rule that establishes that balance is the orphan rule.
Simply stated, the orphan rule says that you can implement a trait
for a type only if the trait or the type is local to your crate. So, you
can implement Debug for your own type, and you can implement
MyNeatTrait for bool, but you cannot implement Debug for bool. If you try,
your code will not compile, and the compiler will tell you that there
are conflicting implementations.

This gets you pretty far; it allows you to implement your own traits
for third-party types and to implement third-party traits for your
own types. However, the orphan rule is not the end of the story.
There are a number of additional implications, caveats, and
exceptions to it that you should be aware of

#### Blanket Implementations

The orphan rule allows you to implement traits over a range of types
with code like impl<T> MyTrait for T where T: and so on. This is a blanket
implementation—it is not limited to just one particular type but
instead applies to a wide range of types. Only the crate that defines a
trait is allowed to write a blanket implementation, and adding a
blanket implementation to an existing trait is considered a breaking
change. If it were not, a downstream crate that contained impl MyTrait
for Foo could suddenly stop compiling just because you update the
crate that defines MyTrait with an error about a conflicting
implementation.

#### Fundamental Types

Some types are so essential that it’s necessary to allow anyone to
implement traits on them, even if this seemingly violates the orphan
rule. These types are marked with the #[fundamental] attribute and
currently include &, &mut, and Box. For the purposes of the orphan rule,
fundamental types may as well not exist—they are effectively erased
before the orphan rule is checked in order to allow you to, for
example, implement IntoIterator for &MyType. With just the orphan rule,
this implementation would not be permitted since it implements a
foreign trait for a foreign type—IntoIterator and & both come from the
standard library. Adding a blanket implementation over a
fundamental type is also considered a breaking change.

#### Covered Implementations

There are some limited cases where we want to allow implementing a
foreign trait for a foreign type, which the orphan rule does not
normally allow. The simplest example of this is when you want to
write something like impl From<MyType> for Vec<i32>. Here, the From trait is
foreign, as is the Vec type, yet there is no danger of violating
coherence. This is because a conflicting implementation could be
added only through a blanket implementation in the standard library
(the standard library cannot otherwise name MyType), which is a
breaking change anyway.


To allow these kinds of implementations, the orphan rule includes
a narrow exemption that permits implementing foreign traits for
foreign types under a very specific set of circumstances. Specifically,
a given impl<P1..=Pn> ForeignTrait<T1..=Tn> for T0 is allowed only if at least
one Ti is a local type and no T before the first such Ti is one of the
generic types P1..=Pn. Generic type parameters (Ps) are allowed to
appear in T0..Ti as long as they are covered by some intermediate
type. A T is covered if it appears as a type parameter to some other
type (like Vec<T>), but not if it stands on its own (just T) or just appears
behind a fundamental type like &T. So, all the implementations in
Listing 2-4 are valid.

```rust
impl<T> From<T> for MyType
impl<T> From<T> for MyType<T>
impl<T> From<MyType> for Vec<T>
impl<T> ForeignTrait<MyType, T> for Vec<T>
```
Listing 2-4: Valid implementations of foreign traits for foreign
types

However, the implementations in Listing 2-5 are invalid.

```rust
impl<T> ForeignTrait for T
impl<T> From<T> for T
impl<T> From<Vec<T>> for T
impl<T> From<MyType<T>> for T
impl<T> From<T> for Vec<T>
impl<T> ForeignTrait<T, MyType> for Vec<T>
```

Listing 2-5: Invalid implementations of foreign traits for foreign
types

This relaxation of the orphan rule complicates the rules for what
constitutes a breaking change when you add a new implementation
for an existing trait. In particular, adding a new implementation to
an existing trait is non-breaking only if it contains at least one new
local type, and that new local type satisfies the rules for the
exemption described earlier. Adding any other new implementation
is a breaking change.


> Note that impl<T> ForeignTrait<LocalType, T> for ForeignType is valid,
but impl<T> ForeignTrait<T, LocalType> for ForeignType is not! This may
seem arbitrary, but without this rule, you could write impl<T>
ForeignTrait<T, LocalType> for ForeignType, and another crate could
write impl<T> ForeignTrait<TheirType, T> for ForeignType, and a conflict
would arise only when the two crates were brought together.
Instead of disallowing this pattern altogether, the orphan
rule requires that your local type come before the type
parameter, which breaks the tie and ensures that if both
crates uphold coherence in isolation, they also uphold it
when combined.


### Trait Bounds

The standard library is flush with trait bounds, whether it’s that the
keys in a HashMap must implement Hash + Eq or that the function given to
thread::spawn must be FnOnce + Send + 'static. When you write generic code
yourself, it will almost certainly include trait bounds, as otherwise
your code cannot do much with the type it is generic over. As you
write more elaborate generic implementations, you’ll find that you
also need more fidelity from your trait bounds, so let’s look at some
of the ways to achieve that.

First and foremost, trait bounds do not have to be of the form T:
Trait where T is some type your implementation or type is generic
over. The bounds can be arbitrary type restrictions and do not even
need to include generic parameters, types of arguments, or local
types. You can write a trait bound like where String: Clone, even though
String: Clone is always true and contains no local types. You can also
write where io::Error: From<MyError<T>>; your generic type parameters do
not need to appear only on the left-hand side. This not only allows
you to express more intricate bounds but also can save you from
needlessly repeating bounds. For example, if your method wants to
construct a HashMap<K, V, S> whose keys are some generic type T and
whose value is a usize, instead of writing the bounds out like where T:
Hash + Eq, S: BuildHasher + Default, you could write where HashMap<T, usize, S>:
FromIterator. This saves you from looking up the exact bounds
requirements for the methods you end up using and more clearly
communicates the “true” requirement of your code. As you can see, it
can also significantly reduce the complexity of your bounds if the
bounds on the underlying trait methods you want to call are
complex.

> Derive Trait
> 
> While #[derive(Trait)] is extremely convenient, in the context of trait bounds, you
should be aware of one subtlety around how it is often implemented. Many #
[derive(Trait)] expansions desugar into impl Trait for Foo<T> where T: Trait. This
is often what you want, but not always. For example, consider what happens if we try to
derive Clone this way for Foo<T> and Foo contains an Arc<T>. Arc implements Clone
regardless of whether T implements Clone, but due to the derived bounds, Foo will
implement Clone only if T does! This isn’t usually too big of an issue, but it does add a
bound where one isn’t needed. If we rename the type to Shared, the problem may
become a little clearer. Imagine how confused a user that has a Shared<NotClone> will
be when the compiler tells them that they cannot clone it! At the time of writing, this is
how #[derive(Clone)] as provided by the standard library works, though this may
change in the future.

Sometimes, you want bounds on associated types of types you’re
generic over. As an example, consider the iterator method flatten,
which takes an iterator that produces items that in turn implement
Iterator and produces an iterator of the items of those inner iterators.
The type it produces, Flatten, is generic over I, which is the type of the
outer iterator. Flatten implements Iterator if I implements Iterator and
the items yielded by I themselves implement IntoIterator. To enable
you to write bounds like this, Rust lets you refer to associated types
of a type using the syntax Type::AssocType. For example, we can refer to
I’s Item type using I::Item. If a type has multiple associated types by the
same name, such as if the trait that provides the associated type is
itself generic (and therefore there are many implementations), you
can disambiguate with the syntax <Type as Trait>::AssocType. Using this,
you can write bounds not only for the outer iterator type but also for
the item type of that outer iterator.

In code that uses generics extensively, you may find that you need
to write a bound that talks about references to a type. This is
normally fine, as you’ll tend to also have a generic lifetime parameter
that you can use as the lifetime for these references. In some cases,
however, you want the bound to say “this reference implements this
trait for any lifetime.” This type of bound is known as a higher￾ranked trait bound, and it’s particularly useful in association with
the Fn traits. For example, say you want to be generic over a function
that takes a reference to a T and returns a reference to inside that T. If
you write F: Fn(&T) -> &U, you need to provide a lifetime for those
references, but you really want to say “any lifetime as long as the
output is the same as the input.” Using a higher-ranked lifetime, you
can write F: for<'a> Fn(&'a T) -> &'a U to say that for any lifetime 'a, the
bound must hold. The Rust compiler is smart enough that it
automatically adds the for when you write Fn bounds with references
like this, which covers the majority of use cases for this feature. The
explicit form is needed so exceedingly rarely that, at the time of
writing, the standard library uses it in just three places—but it does
happen and so is worth knowing about.

To bring all of this together, consider the code in Listing 2-6,
which can be used to implement Debug for any type that can be iterated
over and whose elements are Debug.

```rust
impl Debug for AnyIterable
 where for<'a> &'a Self: IntoIterator,
 for<'a> <&'a Self as IntoIterator>::Item: Debug {
 fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
 f.debug_list().entries(self).finish()
}}
```

Listing 2-6: An excessively generic implementation of Debug for
any iterable collection

You could copy-paste this implementation for pretty much any
collection type and it would “just work.” Of course, you may want a
smarter debug implementation, but this illustrates the power of trait
bounds quite well.

### Maker Traits

Usually, we use traits to denote functionality that multiple types can
support; a Hash type can be hashed by calling hash, a Clone type can be
cloned by calling clone, and a Debug type can be formatted for
debugging by calling fmt. But not all traits are functional in this way.
Some traits, called marker traits, instead indicate a property of the
implementing type. Marker traits have no methods or associated
types and serve just to tell you that a particular type can or cannot be
used in a certain way. For example, if a type implements the Send
marker trait, it is safe to send across thread boundaries. If it does not
implement this marker trait, it isn’t safe to send. There are no
methods associated with this behavior; it’s just a fact about the type.
The standard library has a number of these in the std::marker module,
including Send, Sync, Copy, Sized, and Unpin. Most of these (all except Copy)
are also auto-traits; the compiler automatically implements them for
types unless the type contains something that does not implement
the marker trait.

Marker traits serve an important purpose in Rust: they allow you
to write bounds that capture semantic requirements not directly
expressed in the code. There is no call to send in code that requires
that a type is Send. Instead, the code assumes that the given type is
fine to use in a separate thread, and without marker traits the
compiler would have no way of checking that assumption. It would
be up to the programmer to remember the assumption and read the
code very carefully, which we all know is not something we’d like to
rely on. That path is riddled with data races, segfaults, and other
runtime issues.

Similar to marker traits are marker types. These are unit types
(like struct MyMarker;) that hold no data and have no methods. Marker
types are useful for, well, marking a type as being in a particular
state. They come in handy when you want to make it impossible for a
user to misuse an API. For example, consider a type like SshConnection,
which may or may not have been authenticated yet. You could add a
generic type argument to SshConnection and then create two marker
types: Unauthenticated and Authenticated. When the user first connects,
they get SshConnection<Unauthenticated>. In its impl block, you provide only a
single method: connect. The connect method returns a
SshConnection<Authenticated>, and it’s only in that impl block that you
provide the remaining methods for running commands and such. We
will look at this pattern further in Chapter 3.



### Existential Types

In Rust you very rarely have to specify the types of variables you
declare in the body of a function or the types of generic arguments to
methods that you call. This is because of type inference, where the
compiler decides what type to use based on what type the code the
type appears in evaluates to. The compiler will usually infer types
only for variables and for the arguments (and return types) of
closures; top-level definitions like functions, types, traits, and trait
implementation blocks all require that you explicitly name all types.
There are a couple of reasons for this, but the primary one is that
type inference is much easier when you have at least some known
points to start the inference from. However, it’s not always easy, or
even possible, to fully name a type! For example, if you return a
closure from a function, or an async block from a trait method, its
type does not have a name that you can type into your code.

To handle situations like this, Rust supports existential types.
Chances are, you have already seen existential types in action. All
functions marked as async fn or with a return type of impl Trait have an
existential return type: the signature does not give the true type of
the return value, just a hint that the function returns some type that
implements some set of traits that the caller can rely on. And
crucially, the caller can only rely on the return type implementing
those traits, and nothing else.

> Technically, it isn’t strictly true that the caller relies on the
return type and nothing else. The compiler will also
propagate auto-traits like Send and Sync through impl Trait in
return position. We’ll look at this more in the next chapter.

This behavior is what gives existential types their name: we are
asserting that there exists some concrete type that matches the
signature, and we leave it up to the compiler to find what that type is.
The compiler will usually then go figure that out by applying type
inference on the body of the function.

Not all instances of impl Trait use existential types. If you use impl
Trait in argument position for a function, it’s really just shorthand for
an unnamed generic parameter to that function. For example, fn
foo(s: impl ToString) is mostly just syntax sugar for fn foo<S: ToString>(s: S).
Existential types come in handy particularly when you implement
traits that have associated types. For example, imagine you’re
implementing the IntoIterator trait. It has an associated type IntoIter
that holds the type of the iterator that the type in question can be
turned into. With existential types, you do not need to define a
separate iterator type to use for IntoIter. Instead, you can give the
associated type as impl Iterator<Item = Self::Item> and just write an
expression inside the fn into_iter(self) that evaluates to an Iterator, such
as by using maps and filters over some existing iterator type.

Existential types also provide a feature beyond mere convenience:
they allow you to perform zero-cost type erasure. Instead of
exporting helper types just because they appear in a public signature
somewhere—iterators and futures are common examples of this—
you can use existential types to hide the underlying concrete type.
Users of your interface are shown only the traits that the relevant
type implements, while the concrete type is left as an
implementation detail. Not only does this simplify the interface, but
it also enables you to change that implementation as you wish
without breaking downstream code in the future.


### Summary

This chapter has provided a thorough review of the Rust type system.
We’ve looked both at how the compiler manifests types in memory
and how it reasons about the types themselves. This is important
background material for writing unsafe code, complex application
interfaces, and asynchronous code in later chapters. You’ll also find
that much of the type reasoning from this chapter plays into how you
design Rust code interfaces, which we’ll cover in the next chapter.