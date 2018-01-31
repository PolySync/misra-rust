<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [8.1 A Standard C Environment](#81-a-standard-c-environment)
  - [Rule 1.1 ✔](#rule-11-%E2%9C%94)
  - [Rule 1.2 ✔](#rule-12-%E2%9C%94)
  - [Rule 1.3 ✔](#rule-13-%E2%9C%94)
- [8.2 Unused Code](#82-unused-code)
  - [Rule 2.1 ✔](#rule-21-%E2%9C%94)
  - [Rule 2.2 ✔](#rule-22-%E2%9C%94)
  - [Rule 2.3 ✔](#rule-23-%E2%9C%94)
  - [Rule 2.4 ✔](#rule-24-%E2%9C%94)
  - [Rule 2.5 ✔](#rule-25-%E2%9C%94)
  - [Rule 2.6 ✔](#rule-26-%E2%9C%94)
  - [Rule 2.7 ✔](#rule-27-%E2%9C%94)
- [8.3 Comments](#83-comments)
  - [Rule 3.1 ✔](#rule-31-%E2%9C%94)
  - [Rule 3.2 ✔](#rule-32-%E2%9C%94)
- [8.4 Character sets and lexical conventions](#84-character-sets-and-lexical-conventions)
  - [Rule 4.1 🗙](#rule-41-%F0%9F%97%99)
  - [Rule 4.2 ✔](#rule-42-%E2%9C%94)
- [8.5 Identifiers](#85-identifiers)
  - [Rule 5.1 🗙](#rule-51-%F0%9F%97%99)
  - [Rule 5.2 🗙](#rule-52-%F0%9F%97%99)
  - [Rule 5.3 🗙](#rule-53-%F0%9F%97%99)
  - [Rule 5.4 🗙](#rule-54-%F0%9F%97%99)
  - [Rule 5.5 🗙](#rule-55-%F0%9F%97%99)
  - [Rule 5.6 🗙](#rule-56-%F0%9F%97%99)
  - [Rule 5.7 🗙](#rule-57-%F0%9F%97%99)
  - [Rule 5.8 ✔](#rule-58-%E2%9C%94)
  - [Rule 5.9 ✔](#rule-59-%E2%9C%94)
- [8.6 Types](#86-types)
  - [Rule 6.1 ✔](#rule-61-%E2%9C%94)
  - [Rule 6.2 ✔](#rule-62-%E2%9C%94)
- [8.7 Literals and Constants](#87-literals-and-constants)
  - [Rule 7.1 🗙](#rule-71-%F0%9F%97%99)
  - [Rule 7.2 ✔](#rule-72-%E2%9C%94)
  - [Rule 7.3 ✔](#rule-73-%E2%9C%94)
  - [Rule 7.4 ✔](#rule-74-%E2%9C%94)
- [8.8 Literals and Constants](#88-literals-and-constants)
  - [Rule 8.1 ✔](#rule-81-%E2%9C%94)
  - [Rule 8.2 ✔](#rule-82-%E2%9C%94)
  - [Rule 8.3 ✔](#rule-83-%E2%9C%94)
  - [Rule 8.4 ✔](#rule-84-%E2%9C%94)
  - [Rule 8.5 ✔](#rule-85-%E2%9C%94)
  - [Rule 8.6 ✔](#rule-86-%E2%9C%94)
  - [Rule 8.7 ✔](#rule-87-%E2%9C%94)
  - [Rule 8.8 ✔](#rule-88-%E2%9C%94)
  - [Rule 8.9 ✔](#rule-89-%E2%9C%94)
  - [Rule 8.10 ✔](#rule-810-%E2%9C%94)
  - [Rule 8.11 ✔](#rule-811-%E2%9C%94)
  - [Rule 8.12 ✔](#rule-812-%E2%9C%94)
  - [Rule 8.13 🗙](#rule-813-%F0%9F%97%99)
  - [Rule 8.14 ✔](#rule-814-%E2%9C%94)
- [8.9 Initialization](#89-initialization)
  - [Rule 9.1 ✔](#rule-91-%E2%9C%94)
  - [Rule 9.2 ✔](#rule-92-%E2%9C%94)
  - [Rule 9.3 ✔](#rule-93-%E2%9C%94)
  - [Rule 9.4 ✔](#rule-94-%E2%9C%94)
  - [Rule 9.5 ✔](#rule-95-%E2%9C%94)
- [8.10 The Essential Type Model](#810-the-essential-type-model)
  - [Rule 10.1 🗙](#rule-101-%F0%9F%97%99)
  - [Rule 10.2 ✔](#rule-102-%E2%9C%94)
  - [Rule 10.3 ✔](#rule-103-%E2%9C%94)
  - [Rule 10.4 ✔](#rule-104-%E2%9C%94)
  - [Rule 10.5 ✔](#rule-105-%E2%9C%94)
  - [Rule 10.6 ✔](#rule-106-%E2%9C%94)
  - [Rule 10.7 ✔](#rule-107-%E2%9C%94)
  - [Rule 10.8 ✔](#rule-108-%E2%9C%94)
- [8.11 Pointer Type Conversions](#811-pointer-type-conversions)
  - [Rule 11.1 🗙](#rule-111-%F0%9F%97%99)
  - [Rule 11.2 ✔](#rule-112-%E2%9C%94)
  - [Rule 11.3 ✔](#rule-113-%E2%9C%94)
  - [Rule 11.4 ✔](#rule-114-%E2%9C%94)
  - [Rule 11.5 ✔](#rule-115-%E2%9C%94)
  - [Rule 11.6 ✔](#rule-116-%E2%9C%94)
  - [Rule 11.7 ✔](#rule-117-%E2%9C%94)
  - [Rule 11.8 ✔](#rule-118-%E2%9C%94)
  - [Rule 11.9 ✔](#rule-119-%E2%9C%94)
- [8.12 Expressions](#812-expressions)
  - [Rule 12.1 🗙](#rule-121-%F0%9F%97%99)
  - [Rule 12.2 ✔](#rule-122-%E2%9C%94)
  - [Rule 12.3 ✔](#rule-123-%E2%9C%94)
  - [Rule 12.4 🗙](#rule-124-%F0%9F%97%99)
- [8.13 Side Effects](#813-side-effects)
  - [Rule 13.1 ✔](#rule-131-%E2%9C%94)
  - [Rule 13.2 🗙](#rule-132-%F0%9F%97%99)
  - [Rule 13.3 ✔](#rule-133-%E2%9C%94)
  - [Rule 13.4 ✔](#rule-134-%E2%9C%94)
  - [Rule 13.5 🗙](#rule-135-%F0%9F%97%99)
  - [Rule 13.6 ✔](#rule-136-%E2%9C%94)
- [8.14 Control Statement Expressions](#814-control-statement-expressions)
  - [Rule 14.1 🗙](#rule-141-%F0%9F%97%99)
  - [Rule 14.2 ✔](#rule-142-%E2%9C%94)
  - [Rule 14.3 🗙](#rule-143-%F0%9F%97%99)
  - [Rule 14.4 ✔](#rule-144-%E2%9C%94)
- [8.15 Control Flow](#815-control-flow)
  - [Rule 15.1 🗙](#rule-151-%F0%9F%97%99)
  - [Rule 15.2 🗙](#rule-152-%F0%9F%97%99)
  - [Rule 15.3 ✔](#rule-153-%E2%9C%94)
  - [Rule 15.4 🗙](#rule-154-%F0%9F%97%99)
  - [Rule 15.5 🗙](#rule-155-%F0%9F%97%99)
  - [Rule 15.6 ✔](#rule-156-%E2%9C%94)
  - [Rule 15.7 🗙](#rule-157-%F0%9F%97%99)
- [8.16 Control Flow](#816-control-flow)
  - [Rule 16.1 ✔](#rule-161-%E2%9C%94)
  - [Rule 16.2 ✔](#rule-162-%E2%9C%94)
  - [Rule 16.3 ✔](#rule-163-%E2%9C%94)
  - [Rule 16.4 ✔](#rule-164-%E2%9C%94)
  - [Rule 16.5 ✔](#rule-165-%E2%9C%94)
  - [Rule 16.6 🗙](#rule-166-%F0%9F%97%99)
  - [Rule 16.7 ✔](#rule-167-%E2%9C%94)
- [8.17 Functions](#817-functions)
  - [Rule 17.1 ✔](#rule-171-%E2%9C%94)
  - [Rule 17.2 🗙](#rule-172-%F0%9F%97%99)
  - [Rule 17.3 ✔](#rule-173-%E2%9C%94)
  - [Rule 17.4 🗙](#rule-174-%F0%9F%97%99)
  - [Rule 17.5 ✔](#rule-175-%E2%9C%94)
  - [Rule 17.6 ✔](#rule-176-%E2%9C%94)
  - [Rule 17.7 🗙](#rule-177-%F0%9F%97%99)
  - [Rule 17.8 🗙](#rule-178-%F0%9F%97%99)
- [8.18 Pointers and arrays](#818-pointers-and-arrays)
  - [Rule 18.1 ✔](#rule-181-%E2%9C%94)
  - [Rule 18.2 ✔](#rule-182-%E2%9C%94)
  - [Rule 18.3 🗙](#rule-183-%F0%9F%97%99)
  - [Rule 18.4 ✔](#rule-184-%E2%9C%94)
  - [Rule 18.5 🗙](#rule-185-%F0%9F%97%99)
  - [Rule 18.6 ✔](#rule-186-%E2%9C%94)
  - [Rule 18.7 ✔](#rule-187-%E2%9C%94)
  - [Rule 18.8 ✔](#rule-188-%E2%9C%94)
- [8.19 Overlapping storage](#819-overlapping-storage)
  - [Rule 19.1 ✔](#rule-191-%E2%9C%94)
  - [Rule 19.2 🗙](#rule-192-%F0%9F%97%99)
- [8.20 Overlapping storage](#820-overlapping-storage)
  - [Rule 20.1 🗙](#rule-201-%F0%9F%97%99)
  - [Rule 20.2 🗙](#rule-202-%F0%9F%97%99)
  - [Rule 20.3 ✔](#rule-203-%E2%9C%94)
  - [Rule 20.4 ✔](#rule-204-%E2%9C%94)
  - [Rule 20.5 ✔](#rule-205-%E2%9C%94)
  - [Rule 20.6 🗙](#rule-206-%F0%9F%97%99)
  - [Rule 20.7 ✔](#rule-207-%E2%9C%94)
  - [Rule 20.8 🗙](#rule-208-%F0%9F%97%99)
  - [Rule 20.9 🗙](#rule-209-%F0%9F%97%99)
  - [Rule 20.10 ✔](#rule-2010-%E2%9C%94)
  - [Rule 20.11 ✔](#rule-2011-%E2%9C%94)
  - [Rule 20.12 ✔](#rule-2012-%E2%9C%94)
  - [Rule 20.13 ✔](#rule-2013-%E2%9C%94)
  - [Rule 20.14 ✔](#rule-2014-%E2%9C%94)
- [8.21 Standard libraries](#821-standard-libraries)
  - [Rule 21.1 🗙](#rule-211-%F0%9F%97%99)
  - [Rule 21.2 🗙](#rule-212-%F0%9F%97%99)
  - [Rule 21.3 ✔](#rule-213-%E2%9C%94)
  - [Rule 21.4 ✔](#rule-214-%E2%9C%94)
  - [Rule 21.5 ✔](#rule-215-%E2%9C%94)
  - [Rule 21.6 ✔](#rule-216-%E2%9C%94)
  - [Rule 21.7 ✔](#rule-217-%E2%9C%94)
  - [Rule 21.8 ✔](#rule-218-%E2%9C%94)
  - [Rule 21.9 ✔](#rule-219-%E2%9C%94)
  - [Rule 21.10 ✔](#rule-2110-%E2%9C%94)
  - [Rule 21.11 ✔](#rule-2111-%E2%9C%94)
  - [Rule 21.12 ✔](#rule-2112-%E2%9C%94)
- [8.22 Resources](#822-resources)
  - [Rule 22.1 ✔](#rule-221-%E2%9C%94)
  - [Rule 22.2 ✔](#rule-222-%E2%9C%94)
  - [Rule 22.3 ✔](#rule-223-%E2%9C%94)
  - [Rule 22.4 ✔](#rule-224-%E2%9C%94)
  - [Rule 22.5 ✔](#rule-225-%E2%9C%94)
  - [Rule 22.6 ✔](#rule-226-%E2%9C%94)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


## 8.1 A Standard C Environment

### Rule 1.1 ✔

"The program shall contain no violations of the standard C syntax and
_constraints_, and shall not exceed the implementations translation limits."

_No direct corollary in Rust._

__[See Rule_1_1.rs](./tests/compile-fail/Rule_1_1.rs)__

### Rule 1.2 ✔

"Language extensions should not be used."

_No direct corollary in Rust._

__[See Rule_1_2.rs](./tests/compile-fail/Rule_1_2.rs)__

### Rule 1.3 ✔

"There shall be no occurance of undefined or critical unspecified behavior."

_No direct corollary in Rust._

__[See Rule_1_3.rs](./tests/compile-fail/Rule_1_3.rs)__

## 8.2 Unused Code

### Rule 2.1 ✔

"A project shall not contain unreachable code."

_Enforceable in Rust._

__[See Rule_2_1.rs](./tests/compile-fail/Rule_2_1.rs)__

### Rule 2.2 ✔

"There shall be no _dead code_."

_Enforceable in Rust._

__[See Rule_2_2.rs](./tests/compile-fail/Rule_2_2.rs)__

### Rule 2.3 ✔

"A project should not contain unused type declarations."

_Enforceable in Rust._

__[See Rule_2_3.rs](./tests/compile-fail/Rule_2_3.rs)__

### Rule 2.4 ✔

"A project should not contain unused tag declarations."

_Enforceable in Rust._

__[See Rule_2_4.rs](./tests/compile-fail/Rule_2_4.rs)__

### Rule 2.5 ✔

"A project should not contain unused macro declarations."

_Enforceable in Rust._

__[See Rule_2_5.rs](./tests/compile-fail/Rule_2_5.rs)__

### Rule 2.6 ✔

"A project should not contain unused label declarations."

_Enforceable in Rust._

__[See Rule_2_6.rs](./tests/compile-fail/Rule_2_6.rs)__

### Rule 2.7 ✔

"There should be no unused parameters in functions."

_Enforceable in Rust._

__[See Rule_2_7.rs](./tests/compile-fail/Rule_2_7.rs)__

## 8.3 Comments

### Rule 3.1 ✔

"The character sequences /* and // shall not be used within a comment."

_Enforceable in Rust._

__[See Rule_3_1.rs](./tests/compile-fail/Rule_3_1.rs)__

### Rule 3.2 ✔

"Line-splicing shall not be used in comments."

_Enforceable in Rust._

__[See Rule_3_2.rs](./tests/compile-fail/Rule_3_2.rs)__

## 8.4 Character sets and lexical conventions

### Rule 4.1 ✔

"Octal and hexadecimal escape sequences shall be terminated."

_Escape sequences are terminated implicitly, ambuguous escapes are not allowed._
_All ASCII and byte escapes require exactly 2 hexadecimal digits._

__[See Rule_4_1.rs](./tests/compile-fail/Rule_4_1.rs)__

### Rule 4.2 ✔

"Trigraphs should not be used."

_No direct corollary in Rust._

__[See Rule_4_2.rs](./tests/compile-fail/Rule_4_2.rs)__

## 8.5 Identifiers

### Rule 5.1 🗙

"External identifiers shall be distinct."

_This rule is not strictly enforcable in Rust._

__[See Rule_5_1.rs](./tests/compile-fail/Rule_5_1.rs)__

### Rule 5.2 🗙

"Identifiers declared in the same scope and namespace shall be shall be
distinct."

_This rule is not strictly enforcable in Rust._

__[See Rule_5_2.rs](./tests/compile-fail/Rule_5_2.rs)__

### Rule 5.3 🗙

"An identifier declared in an inner scope shall not hide an identifier declared
in an outer scope."

_This rule is not strictly enforcable in Rust. Shadowing is allowed._

__[See Rule_5_3.rs](./tests/compile-fail/Rule_5_3.rs)__

### Rule 5.4 🗙

"Macro identifiers shall be distinct."

_This rule is not strictly enforcable in Rust. Shadowing is allowed._

__[See Rule_5_4.rs](./tests/compile-fail/Rule_5_4.rs)__

### Rule 5.5 🗙

"Identifiers shall be distinct from macro names."

_This rule is not strictly enforcable in Rust but macros aren't mistakable as_
_other identifiers in Rust in the same way because of the `!` character_
_requirement. Shadowing is allowed._

__[See Rule_5_5.rs](./tests/compile-fail/Rule_5_5.rs)__

### Rule 5.6 🗙

"A typedef name shall be a unique identifier."

_This rule is not strictly enforcable in Rust. Shadowing is allowed._

__[See Rule_5_6.rs](./tests/compile-fail/Rule_5_6.rs)__

### Rule 5.7 🗙

"A tag name shall be a unique identifier."

_This rule is not strictly enforcable in Rust. Shadowing is allowed._

__[See Rule_5_7.rs](./tests/compile-fail/Rule_5_7.rs)__

### Rule 5.8 ✔

"Identifiers that define objects or functions with external linkage shall be
unique."

_Needs more research. This rule may not be strictly enforcable in Rust as_
_shadowing is allowed but the example at least causes a compile error._

__[See Rule_5_8.rs](./tests/compile-fail/Rule_5_8.rs)__

### Rule 5.9 ✔

"Identifiers that define objects or functions with internal linkage shall be
unique."

_Needs more research. This rule may not be strictly enforcable in Rust as_
_shadowing is allowed but the example at least causes a compile error._

__[See Rule_5_9.rs](./tests/compile-fail/Rule_5_9.rs)__

## 8.6 Types

### Rule 6.1 ✔

"Bit-fields shall only be declared with an appropriate type."

_No direct corollary in Rust._

__[See Rule_6_1.rs](./tests/compile-fail/Rule_6_1.rs)__

### Rule 6.2 ✔

"Single-bit named fields shall not be of a signed type."

_No direct corollary in Rust._

__[See Rule_6_2.rs](./tests/compile-fail/Rule_6_2.rs)__

## 8.7 Literals and Constants

### Rule 7.1 🗙

"Octal constants shall not be used."

_This rule is not strictly enforcable in Rust though octal constants are_
_differentiatable from decimal constants because they require the `0o` prefix._

__[See Rule_7_1.rs](./tests/compile-fail/Rule_7_1.rs)__

### Rule 7.2 ✔

"A "u" or "U" suffix shall be applied to all integer constants that are
represented in an unsigned type.

_This rule is not strictly enforcable in Rust, there is no 'U' or 'u' suffix_
_though the language enforces that signed and unsigned types are not_
_interchangeable without being explicit._

__[See Rule_7_2.rs](./tests/compile-fail/Rule_7_2.rs)__

### Rule 7.3 ✔

"The lowercase 'l' shall not be used in a literal suffix."

_No direct corollary in Rust._

__[See Rule_7_3.rs](./tests/compile-fail/Rule_7_3.rs)__

### Rule 7.4 ✔

"A string literal shall not be assigned to an object unless the object's type
is a 'pointer to const-qualified _char_'."

_This rule is not strictly enforcable in Rust as it doesn't apply in a literal_
_sense. Rust will allow a mutable variable representing a string literal while_
_at the same time disallowing unsafe access or modifications to it._

__[See Rule_7_4.rs](./tests/compile-fail/Rule_7_4.rs)__

## 8.8 Literals and Constants

### Rule 8.1 ✔

"Types shall be explicitly specified."

_No direct corollary in Rust._

__[See Rule_8_1.rs](./tests/compile-fail/Rule_8_1.rs)__

### Rule 8.2 ✔

"Function types shall be in prototype form with named parameters."

_No direct corollary in Rust._

__[See Rule_8_2.rs](./tests/compile-fail/Rule_8_2.rs)__

### Rule 8.3 ✔

"All declarations of an object or function shall use the same names and type
qualifiers."

_No direct corollary in Rust._

__[See Rule_8_3.rs](./tests/compile-fail/Rule_8_3.rs)__

### Rule 8.4 ✔

"A compatible declaration shall be visible when an object or function with
external linkage is defined."

_No direct corollary in Rust._

__[See Rule_8_4.rs](./tests/compile-fail/Rule_8_4.rs)__

### Rule 8.5 ✔

"An external object or function shall be declared once in one and only one
file."

_No direct corollary in Rust._

__[See Rule_8_5.rs](./tests/compile-fail/Rule_8_5.rs)__

### Rule 8.6 ✔

"An identifier with external linkage shall have exactly one external
definition."

_No direct corollary in Rust._

__[See Rule_8_6.rs](./tests/compile-fail/Rule_8_6.rs)__

### Rule 8.7 ✔

"Functions and objects should not be defined with external linkage if they
are referenced in only one translation unit."

_No direct corollary in Rust._

__[See Rule_8_7.rs](./tests/compile-fail/Rule_8_7.rs)__

### Rule 8.8 ✔

"The static storage class specifier shall be used in all declarations of objects
and functions that have internal linkage."

_No direct corollary in Rust._

__[See Rule_8_8.rs](./tests/compile-fail/Rule_8_8.rs)__

### Rule 8.9 ✔

"An object should be defined at block scope if its identifier only appears in
a single function." (Don't use globals if a local variable will work instead.)

_No direct corollary in Rust._

__[See Rule_8_9.rs](./tests/compile-fail/Rule_8_9.rs)__

### Rule 8.10 ✔

"An _inline_ function shall be declared with the static storage class."

_No direct corollary in Rust._

__[See Rule_8_10.rs](./tests/compile-fail/Rule_8_10.rs)__

### Rule 8.11 ✔

"When an array with external linkage is declared, its size should be explicitly
specified."

_Enforceable in Rust._

__[See Rule_8_11.rs](./tests/compile-fail/Rule_8_11.rs)__

### Rule 8.12 ✔

"Within an enumerator list, the value of an implicitly specified enmueration
constnat shall be unique."

_Enforceable in Rust._

__[See Rule_8_12.rs](./tests/compile-fail/Rule_8_12.rs)__

### Rule 8.13 🗙

"A pointer should point to a const-qualified type whenever possible."

_This is not strictly enforceable in Rust._

__[See Rule_8_13.rs](./tests/compile-fail/Rule_8_13.rs)__

### Rule 8.14 ✔

"The _restrict_ type qualifier shall not be used."

_No direct corollary in Rust._

__[See Rule_8_14.rs](./tests/compile-fail/Rule_8_14.rs)__

## 8.9 Initialization

### Rule 9.1 ✔

"The value of an object with automatic storage duration shall not be read before
it is set."

_Enforceable in Rust._

__[See Rule_9_1.rs](./tests/compile-fail/Rule_9_1.rs)__

### Rule 9.2 ✔

"The initializer for an aggregate or union shall be enclosed in braces."

_No direct corollary in Rust._

__[See Rule_9_2.rs](./tests/compile-fail/Rule_9_2.rs)__

### Rule 9.3 ✔

"Arrays shall not be partially initialized."

_Enforceable in Rust._

__[See Rule_9_3.rs](./tests/compile-fail/Rule_9_3.rs)__

### Rule 9.4 ✔

"An element of an object shall not be initialized more than once."

_Enforceable in Rust._

__[See Rule_9_4.rs](./tests/compile-fail/Rule_9_4.rs)__

### Rule 9.5 ✔

"Where designated initializers are used to initialize an array object the size
of the array shall be specified explicitly."

_Enforceable in Rust._

__[See Rule_9_5.rs](./tests/compile-fail/Rule_9_5.rs)__

## 8.10 The Essential Type Model

### Rule 10.1 🗙

"Operands shall not be of an inappropriate essential type."

_This is not strictly enforceable in Rust. For instance bitwise operations_
_on signed integers are allowed._

__[See Rule_10_1.rs](./tests/compile-fail/Rule_10_1.rs)__

### Rule 10.2 ✔

"Expressions of essentially character type shall not be used inappropriately
in addition and subtraction operations."

_Enforceable in Rust._

__[See Rule_10_2.rs](./tests/compile-fail/Rule_10_2.rs)__

### Rule 10.3 ✔

"The value od an experssionshall not be assigned to an object with a narrower
essential type or of a different essential type category."

_Enforceable in Rust._

__[See Rule_10_3.rs](./tests/compile-fail/Rule_10_3.rs)__

### Rule 10.4 ✔

"Both operands of an operator in which the usual arithmetic conversions are
performed shall have the same essential type category."

_Enforceable in Rust._

__[See Rule_10_4.rs](./tests/compile-fail/Rule_10_4.rs)__

### Rule 10.5 ✔

"The value of an expression should not be cast to an inappropriate essential
type."

_Enforceable in Rust._

__[See Rule_10_5.rs](./tests/compile-fail/Rule_10_5.rs)__

### Rule 10.6 ✔

"The value of a composite expression shall not be assigned to an object
with wider essential type."

_Enforceable in Rust._

__[See Rule_10_6.rs](./tests/compile-fail/Rule_10_6.rs)__

### Rule 10.7 ✔

"If a composite expression is used as one operand of an operator in which the
usual arithmetic conversions are performed then the other operand shall not have
wider essential type."

_Enforceable in Rust._

__[See Rule_10_7.rs](./tests/compile-fail/Rule_10_7.rs)__

### Rule 10.8 ✔

"The value of a composite expression shall not be cast to a different essential
type category or a wider essential type."

_Enforceable in Rust._

__[See Rule_10_8.rs](./tests/compile-fail/Rule_10_8.rs)__

## 8.11 Pointer Type Conversions

### Rule 11.1 🗙

"Conversions shall not be performed between a pointer to a function and any
other type."

_This is not strictly enforceable in Rust._

__[See Rule_11_1.rs](./tests/compile-fail/Rule_11_1.rs)__

### Rule 11.2 ✔

"Conversions shall not be performed between a pointer to an incomplete type and
any other type."

_Enforceable in Rust._

__[See Rule_11_2.rs](./tests/compile-fail/Rule_11_2.rs)__

### Rule 11.3 ✔

"A cast shall not be performed between a pointer to object type and a pointer
to different object type."

_You may define the `From` trait to facilitate this behavior but by default,_
_this behavior is not allowed for custom types._

_Enforceable in Rust._

__[See Rule_11_3.rs](./tests/compile-fail/Rule_11_3.rs)__

### Rule 11.4 ✔

"A conversion should not be performed between a pointer to object and integer
type."

_Enforceable in Rust._

__[See Rule_11_4.rs](./tests/compile-fail/Rule_11_4.rs)__

### Rule 11.5 ✔

"A conversion should not be performed from pointer to void into pointer to
object."

_No direct corollary in Rust._

__[See Rule_11_5.rs](./tests/compile-fail/Rule_11_5.rs)__

### Rule 11.6 ✔

"A conversion should not be performed from pointer to void and an arithmetic
type."

_No direct corollary in Rust._

__[See Rule_11_6.rs](./tests/compile-fail/Rule_11_6.rs)__

### Rule 11.7 ✔

"A conversion should not be performed from pointer to object and a non-integer
arithmetic type."

_Enforceable in Rust._

__[See Rule_11_7.rs](./tests/compile-fail/Rule_11_7.rs)__

### Rule 11.8 ✔

"A cast shall not remove any const or volatile qualification from the type
pointed to by a pointer."

_Enforceable in Rust._

__[See Rule_11_8.rs](./tests/compile-fail/Rule_11_8.rs)__

### Rule 11.9 ✔

"The macro NULL shall be the only permitted form of integer null pointer
constant."

_No direct corollary in Rust._

__[See Rule_11_9.rs](./tests/compile-fail/Rule_11_9.rs)__

## 8.12 Expressions

### Rule 12.1 🗙

"The precedence of operators within expressions should be made explicit"

_This is not strictly enforceable in Rust._

__[See Rule_12_1.rs](./tests/compile-fail/Rule_12_1.rs)__

### Rule 12.2 ✔

"The right hand operand of a shift operator shall lie in the range zero to one
less than the width in bits of the essential type of the left hand operand."

_Enforceable in Rust._

__[See Rule_12_2.rs](./tests/compile-fail/Rule_12_2.rs)__

### Rule 12.3 ✔

"The comma operator should not be used."

_No direct corollary in Rust._

__[See Rule_12_3.rs](./tests/compile-fail/Rule_12_3.rs)__

### Rule 12.4 🗙

"Evaluation of constant expressions should not lead to unsigned integer wrap
around."

_This is not strictly enforceable in Rust._

__[See Rule_12_4.rs](./tests/compile-fail/Rule_12_4.rs)__

## 8.13 Side Effects

### Rule 13.1 ✔

"Initializer lists shall not contain persistent side effects."

_Enforceable in Rust._

__[See Rule_13_1.rs](./tests/compile-fail/Rule_13_1.rs)__

### Rule 13.2 🗙

"The value of an expression and its persistent side effects shall be the same
under all permitted evaluation orders."

_This is not strictly enforceable in Rust._

__[See Rule_13_2.rs](./tests/compile-fail/Rule_13_2.rs)__

### Rule 13.3 ✔

"A full expression containing an increment (++) or decrement (--✔) operator
should have no other potential side effects other than that caused by the
incremenet or decrement operator."

_No direct corollary in Rust._

__[See Rule_13_3.rs](./tests/compile-fail/Rule_13_3.rs)__

### Rule 13.4 ✔

"The result of an assignment operator should not be _used_."

_Enforceable in Rust in the sense that `()` is generally unusable in ways that_
_mistakes can be made in C. If `()` is actually desired it can be used as that._

__[See Rule_13_4.rs](./tests/compile-fail/Rule_13_4.rs)__

### Rule 13.5 🗙

"The right hand operand of a logical && or || operator shall not contain
persistent side effects."

_This is not strictly enforceable in Rust._

__[See Rule_13_5.rs](./tests/compile-fail/Rule_13_5.rs)__

### Rule 13.6 ✔

"The right hand operand of a logical && or || operator shall not contain
persistent side effects."

_No direct corollary in Rust._

__[See Rule_13_6.rs](./tests/compile-fail/Rule_13_6.rs)__

## 8.14 Control Statement Expressions

### Rule 14.1 🗙

"A loop counter shall not have essentially floating type."

_This is not strictly enforceable in Rust._

__[See Rule_14_1.rs](./tests/compile-fail/Rule_14_1.rs)__

### Rule 14.2 ✔

"A _for_ loop shall be well-formed."

_Enforceable in Rust._

__[See Rule_14_2.rs](./tests/compile-fail/Rule_14_2.rs)__

### Rule 14.3 🗙

"Controlling expressions shall not be invariant."

_This is not strictly enforceable in Rust._

__[See Rule_14_3.rs](./tests/compile-fail/Rule_14_3.rs)__

### Rule 14.4 ✔

"The controlling expression of an _if_ statement and the controlling expression
of an iteration-statement shall have essentially Boolean type."

_Enforceable in Rust._

__[See Rule_14_4.rs](./tests/compile-fail/Rule_14_4.rs)__

## 8.15 Control Flow

### Rule 15.1 🗙

"The _goto_ statement shall not be used."

_Assuming loop labels are equivalent, this is not strictly enforceable in Rust._

__[See Rule_15_1.rs](./tests/compile-fail/Rule_15_1.rs)__

### Rule 15.2 🗙

"The _goto_ statement shall jump to a label declared later in the same
function."

_This is not strictly enforceable in Rust._

__[See Rule_15_2.rs](./tests/compile-fail/Rule_15_2.rs)__

### Rule 15.3 ✔

"Any label referenced by a _goto_ statement shall be declared in the same block,
or in any block enclosing the goto statement."

_Enforceable in Rust._

__[See Rule_15_3.rs](./tests/compile-fail/Rule_15_3.rs)__

### Rule 15.4 🗙

"There should be no more than one _break_ or _goto_ statement used to terminate
any iteration statement."

_This is not strictly enforceable in Rust._

__[See Rule_15_4.rs](./tests/compile-fail/Rule_15_4.rs)__

### Rule 15.5 🗙

"A function should have a single point of exit at the end."

_This is not strictly enforceable in Rust._

__[See Rule_15_5.rs](./tests/compile-fail/Rule_15_5.rs)__

### Rule 15.6 ✔

"The body of an _iteration-statement_ or _selection-statement_ shall be a
_compound-statement_."

_Enforceable in Rust._

__[See Rule_15_6.rs](./tests/compile-fail/Rule_15_6.rs)__

### Rule 15.7 🗙

"All _if ... else_ statements shall be terminated with and _else_ statement."

_This is not strictly enforceable in Rust._

__[See Rule_15_7.rs](./tests/compile-fail/Rule_15_7.rs)__

## 8.16 Control Flow

### Rule 16.1 ✔

"All switch statements shall be well-formed."

_This is not strictly enforceable in Rust as MISRA's idea of well-formed_
_deviates from Rust's (see Rule 16.6 and 16.7)._

__[See Rule_16_1.rs](./tests/compile-fail/Rule_16_1.rs)__

### Rule 16.2 ✔

"A _switch label_ shall only be used when the most closely-enclosing compound
statement is the body of a _switch_ statement."

_Enforceable in Rust._

__[See Rule_16_2.rs](./tests/compile-fail/Rule_16_2.rs)__

### Rule 16.3 ✔

"An unconditional _break_ statement shall terminate every _switch-clause_."

_No direct corollary in Rust._

__[See Rule_16_3.rs](./tests/compile-fail/Rule_16_3.rs)__

### Rule 16.4 ✔

"Every _switch_ statement shall have a _default_ label."

_Enforceable in Rust._

__[See Rule_16_4.rs](./tests/compile-fail/Rule_16_4.rs)__

### Rule 16.5 ✔

"A _default_ label shall appear as either the first or the last _switch label_
of a _switch_ statement."

_Enforceable in Rust._

__[See Rule_16_5.rs](./tests/compile-fail/Rule_16_5.rs)__

### Rule 16.6 🗙

"Every _switch_ statement shall have at least two _switch-clauses_."

_This is not strictly enforceable in Rust._

__[See Rule_16_6.rs](./tests/compile-fail/Rule_16_6.rs)__

### Rule 16.7 ✔

"A _switch-expression_ shall not have _essentially Boolean type_."

_Enforceable in Rust._

__[See Rule_16_7.rs](./tests/compile-fail/Rule_16_7.rs)__

## 8.17 Functions

### Rule 17.1 ✔

"The features of `<stdarg.h>` shall not be used."

_No direct corollary in Rust._

__[See Rule_17_1.rs](./tests/compile-fail/Rule_17_1.rs)__

### Rule 17.2 🗙

"Functions shall not call themselves, either directly or indirectly."

_This is not strictly enforceable in Rust._

__[See Rule_17_2.rs](./tests/compile-fail/Rule_17_2.rs)__

### Rule 17.3 ✔

"A function shall not be declared implicitly."

_No direct corollary in Rust. There isn't really the same concept_
_of function prototypes._

__[See Rule_17_3.rs](./tests/compile-fail/Rule_17_3.rs)__

### Rule 17.4 🗙

"All exit paths from a function with non-_void_ return type shall have an
explicit return statement with an expression."

_Enforceable in Rust._

__[See Rule_17_4.rs](./tests/compile-fail/Rule_17_4.rs)__

### Rule 17.5 ✔

"The function argument corresponding to a parameter declared to have an array
type shall have an appropriate number of elements."

_Enforceable in Rust._

__[See Rule_17_5.rs](./tests/compile-fail/Rule_17_5.rs)__

### Rule 17.6 ✔

"The declaration of an array parameter shall not contain the _static_ keyword
between the __[]__."

_No direct corollary in Rust._

__[See Rule_17_6.rs](./tests/compile-fail/Rule_17_6.rs)__

### Rule 17.7 🗙

"The value returned by a function having non-_void_ return type shall be used."

_This is not strictly enforceable in Rust._

__[See Rule_17_7.rs](./tests/compile-fail/Rule_17_7.rs)__

### Rule 17.8 🗙

"A function parameter shall not be modified."

__[See Rule_17_8.rs](./tests/compile-fail/Rule_17_8.rs)__

_This is not strictly enforceable in Rust._

## 8.18 Pointers and arrays

### Rule 18.1 ✔

"A pointer resulting from arithmetic on a pointer operand shall address an
element of the same array as that pointer operand."

_Enforceable in Rust. Not because the value is out of bounds but because_
_pointer/reference arithmatic isn't allowed._

__[See Rule_18_1.rs](./tests/compile-fail/Rule_18_1.rs)__

### Rule 18.2 ✔

"Subtraction beteen pointers shall only be applies to pointers that address
elements of the same array."

_Enforceable in Rust. Not because the value is out of bounds but because_
_pointer/reference arithmatic isn't allowed._

__[See Rule_18_2.rs](./tests/compile-fail/Rule_18_2.rs)__

### Rule 18.3 🗙

"The relational operators _>_, _>=_, _<_ and _<=_ shall not be applied to
objects of pointer type except where they point to the same object."

_This is not strictly enforceable in Rust._

__[See Rule_18_3.rs](./tests/compile-fail/Rule_18_3.rs)__

### Rule 18.4 ✔

"The _+_, _-_, _+=_ and _-=_ operators should not be applied to an expression of
pointer type."

_Enforceable in Rust._

__[See Rule_18_4.rs](./tests/compile-fail/Rule_18_4.rs)__

### Rule 18.5 🗙

"Declarations should contain no more thaan two levels of pointer nesting."

_This is not strictly enforceable in Rust._

__[See Rule_18_5.rs](./tests/compile-fail/Rule_18_5.rs)__

### Rule 18.6 ✔

"The address of an object with automatic storage shall not be copied to another
object that persists after the first object has ceased to exist."

_Enforceable in Rust._

__[See Rule_18_6.rs](./tests/compile-fail/Rule_18_6.rs)__

### Rule 18.7 ✔

"Flexible array members shall not be declared."

_Enforceable in Rust._

__[See Rule_18_7.rs](./tests/compile-fail/Rule_18_7.rs)__

### Rule 18.8 ✔

"Variable-length array types shall not be used."

_Enforceable in Rust._

__[See Rule_18_8.rs](./tests/compile-fail/Rule_18_8.rs)__

## 8.19 Overlapping storage

### Rule 19.1 ✔

"An object shall not be assigned or copied to an overlapping object/"

_Enforceable in Rust._

__[See Rule_19_1.rs](./tests/compile-fail/Rule_19_1.rs)__

### Rule 19.2 🗙

"The _union_ keyword should not be used."

_This is not strictly enforceable in Rust._

__[See Rule_19_2.rs](./tests/compile-fail/Rule_19_1.rs)__

## 8.20 Overlapping storage

### Rule 20.1 🗙

"_#include_ directives should only be preceded by preprocessor directives or
comments."

_This is not strictly enforceable in Rust. (sub `use` for `include`)_

__[See Rule_20_1.rs](./tests/compile-fail/Rule_20_1.rs)__

### Rule 20.2 🗙

"The _'_, _"_ or _\_ characters and the _/*_ or _//_ character sequecnce shall
not occur in a _header file_ name."

_This is not strictly enforceable in Rust._

__[See Rule_20_2.rs](./tests/compile-fail/Rule_20_2.rs)__

### Rule 20.3 ✔

"The _#include_ directive shall be followed by either a `<filename>` or
"filname" sequence."

_No direct corollary in Rust._

__[See Rule_20_3.rs](./tests/compile-fail/Rule_20_3.rs)__

### Rule 20.4 ✔

"A macro shall not be defined with the same name as a keyword"

_Enforceable in Rust._

__[See Rule_20_4.rs](./tests/compile-fail/Rule_20_4.rs)__

### Rule 20.5 ✔

"_#undef_ should not be used."

_No direct corollary in Rust._

__[See Rule_20_5.rs](./tests/compile-fail/Rule_20_5.rs)__

### Rule 20.6 🗙

"Tokens that look like a preprocessing directive shall not occur within a macro
argument."

_This is not strictly enforceable in Rust._

__[See Rule_20_6.rs](./tests/compile-fail/Rule_20_6.rs)__

### Rule 20.7 ✔

"Expressions resulting from the expansion of macro parameters shall be enclosed
in parentheses."

_Enforceable in Rust._

__[See Rule_20_7.rs](./tests/compile-fail/Rule_20_7.rs)__

### Rule 20.8 🗙

"The controlling expression of a _#if_ or _#elif_ preprocessing directive shall
evaluate to 0 or 1"

_This is not strictly enforceable in Rust, the cfg! macro allows for more_
_sophistication._

__[See Rule_20_8.rs](./tests/compile-fail/Rule_20_8.rs)__

### Rule 20.9 🗙

"All identifiers used in the controlling expression of _#if_ or _#elif_
preprocessing directives shall be _#define_'d before evaluation."

_This is not strictly enforceable in Rust, the cfg! macro allows for more_
_sophistication._

__[See Rule_20_9.rs](./tests/compile-fail/Rule_20_9.rs)__

### Rule 20.10 ✔

"The _#_ and _##_ preprocessor should not be used."

_No direct corollary in Rust._

__[See Rule_20_10.rs](./tests/compile-fail/Rule_20_10.rs)__

### Rule 20.11 ✔

"A macro paramete immediately following a _#_ operator shall not immediately
be followed by a _##_ operator."

_No direct corollary in Rust._

__[See Rule_20_11.rs](./tests/compile-fail/Rule_20_11.rs)__

### Rule 20.12 ✔

"A macro parameter used as an operand to the _#_ or _##_ operators, which is
itself subject to further macro replacement, shall only be used as an operand
to these parameters."

_No direct corollary in Rust._

__[See Rule_20_12.rs](./tests/compile-fail/Rule_20_12.rs)__

### Rule 20.13 ✔

"A line whose first token is _#_ shall be a valid preprocessing directive."

_No direct corollary in Rust._

__[See Rule_20_13.rs](./tests/compile-fail/Rule_20_13.rs)__

### Rule 20.14 ✔

"All _#else_, _#elif_ and _#endif_ preprocessor directives shall reside in the
same file as the _#if_, _#ifdef_ or _#ifndef_ directive to which they are
related."

_No direct corollary in Rust._

__[See Rule_20_14.rs](./tests/compile-fail/Rule_20_14.rs)__

## 8.21 Standard libraries

### Rule 21.1 🗙

"_#define_ and _#undef_ shall not be used on a reserved identifier or reserved
macro name."

_This is not strictly enforceable in Rust._

__[See Rule_21_1.rs](./tests/compile-fail/Rule_21_1.rs)__

### Rule 21.2 🗙

"A reserved identifier or macro name shall not be declared."

_This is not strictly enforceable in Rust._

__[See Rule_21_2.rs](./tests/compile-fail/Rule_21_2.rs)__

### Rule 21.3 ✔

"The memory allocation and deallocatin functions of _<stdlib.h>_ shall not be
used."

_No direct corollary in Rust._

__[See Rule_21_3.rs](./tests/compile-fail/Rule_21_3.rs)__

### Rule 21.4 ✔

"The standard _header file <setjmp.h>_ shall not be used."

_No direct corollary in Rust._

__[See Rule_21_4.rs](./tests/compile-fail/Rule_21_4.rs)__

### Rule 21.5 ✔

"The standard _header file <signal.h>_ shall not be used."

_No direct corollary in Rust._

__[See Rule_21_5.rs](./tests/compile-fail/Rule_21_5.rs)__

### Rule 21.6 ✔

"The Standard Library input/output functions shall not be used."

_If `std` or `io` were comprable this is not strictly enforceable in Rust._
_Otherwise there is no direct corollarry._

__[See Rule_21_6.rs](./tests/compile-fail/Rule_21_6.rs)__

### Rule 21.7 ✔

"The _atof_, _atoi_, _atol_ and _atoll_ functions of _<stdlib.h>_ shall not be
used."

_If the implementation of `From` for those types were comprable this is_
_not strictly enforceable in Rust. Otherise there is no direct corollary._

__[See Rule_21_7.rs](./tests/compile-fail/Rule_21_7.rs)__

### Rule 21.8 ✔

"The library functions _abort_, _exit_, _getenv_ and _system_ of _<stdlib.h>_
shall not be used."

_If the corresponding functionality of those functions were comprable this is_
_not strictly enforceable in Rust. Otherise there is no direct corollary._

__[See Rule_21_8.rs](./tests/compile-fail/Rule_21_8.rs)__

### Rule 21.9 ✔

"The library functions _bsearch_ and _qsort_ of _<stdlib.h>_
shall not be used."

_If the corresponding functionality of those functions were comprable this is_
_not strictly enforceable in Rust. Otherise there is no direct corollary._

__[See Rule_21_9.rs](./tests/compile-fail/Rule_21_9.rs)__

### Rule 21.10 ✔

"The Standard Library time and date functions shall not be used."

_If the corresponding functionality was comprable this is_
_not strictly enforceable in Rust. Otherise there is no direct corollary._

__[See Rule_21_10.rs](./tests/compile-fail/Rule_21_10.rs)__

### Rule 21.11 ✔

"The standard _header file _<tgmath.h>_ shall not be used."

_No direct corollary in Rust._

__[See Rule_21_11.rs](./tests/compile-fail/Rule_21_11.rs)__

### Rule 21.12 ✔

"The exception handling features of _<fenv.h>_ should not be used."

_No direct corollary in Rust._

__[See Rule_21_12.rs](./tests/compile-fail/Rule_21_12.rs)__

## 8.22 Resources

### Rule 22.1 ✔

"All resources obtained dynamically by means of Standard Library functions
shall be explicitly released."

_If the `drop` was comprable this is_
_not strictly enforceable in Rust. Otherise there is no direct corollary._

__[See Rule_22_1.rs](./tests/compile-fail/Rule_22_1.rs)__

### Rule 22.2 ✔

"A block of memory shall only be freed if it was allocated by means of a
Standard Library function"

_No direct corollary in Rust._

__[See Rule_22_2.rs](./tests/compile-fail/Rule_22_2.rs)__

### Rule 22.3 ✔

"The same file shall not be open for read and write access at the same time
on different streams."

_This is not strictly enforceable in Rust._

__[See Rule_22_3.rs](./tests/compile-fail/Rule_22_3.rs)__

### Rule 22.4 ✔

"The same file shall not be open for read and write access at the same time
on different streams."

_This is not strictly enforceable in Rust._

__[See Rule_22_4.rs](./tests/compile-fail/Rule_22_4.rs)__

### Rule 22.5 ✔

"A pointer to _FILE_ object shall not be dereferenced."

_If the resulting file handle in the result retured from `File::open` is_
_comprable, this is enforceable in Rust. Otherwise there is no direct_
corollary in Rust._

__[See Rule_22_5.rs](./tests/compile-fail/Rule_22_5.rs)__

### Rule 22.6 ✔

"The value of a pointer to a `FILE` shall not be used after the associated
stream has been closed."

_If the resulting file handle in the result retured from `File::open` is_
_comprable, this is enforceable in Rust. Otherwise there is no direct_
corollary in Rust._

__[See Rule_22_6.rs](./tests/compile-fail/Rule_22_6.rs)__