# MISRA:C
*Guidelines for the use of the C language in critical systems.*

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [8.1 A Standard C Environment](#81-a-standard-c-environment)
  - [Rule 1.1](#rule-11)
  - [Rule 1.2](#rule-12)
  - [Rule 1.3](#rule-13)
- [8.2 Unused Code](#82-unused-code)
  - [Rule 2.1](#rule-21)
  - [Rule 2.2](#rule-22)
  - [Rule 2.3](#rule-23)
  - [Rule 2.4](#rule-24)
  - [Rule 2.5](#rule-25)
  - [Rule 2.6](#rule-26)
  - [Rule 2.7](#rule-27)
- [8.3 Comments](#83-comments)
  - [Rule 3.1](#rule-31)
  - [Rule 3.2](#rule-32)
- [8.4 Character sets and lexical conventions](#84-character-sets-and-lexical-conventions)
  - [Rule 4.1](#rule-41)
  - [Rule 4.2](#rule-42)
- [8.5 Identifiers](#85-identifiers)
  - [Rule 5.1](#rule-51)
  - [Rule 5.2](#rule-52)
  - [Rule 5.3](#rule-53)
  - [Rule 5.4](#rule-54)
  - [Rule 5.5](#rule-55)
  - [Rule 5.6](#rule-56)
  - [Rule 5.7](#rule-57)
  - [Rule 5.8](#rule-58)
  - [Rule 5.9](#rule-59)
- [8.6 Types](#86-types)
  - [Rule 6.1](#rule-61)
  - [Rule 6.2](#rule-62)
- [8.7 Literals and Constants](#87-literals-and-constants)
  - [Rule 7.1](#rule-71)
  - [Rule 7.2](#rule-72)
  - [Rule 7.3](#rule-73)
  - [Rule 7.4](#rule-74)
- [8.8 Literals and Constants](#88-literals-and-constants)
  - [Rule 8.1](#rule-81)
  - [Rule 8.2](#rule-82)
  - [Rule 8.3](#rule-83)
  - [Rule 8.4](#rule-84)
  - [Rule 8.5](#rule-85)
  - [Rule 8.6](#rule-86)
  - [Rule 8.7](#rule-87)
  - [Rule 8.8](#rule-88)
  - [Rule 8.9](#rule-89)
  - [Rule 8.10](#rule-810)
  - [Rule 8.11](#rule-811)
  - [Rule 8.12](#rule-812)
  - [Rule 8.13](#rule-813)
  - [Rule 8.14](#rule-814)
- [8.9 Initialization](#89-initialization)
  - [Rule 9.1](#rule-91)
  - [Rule 9.2](#rule-92)
  - [Rule 9.3](#rule-93)
  - [Rule 9.4](#rule-94)
  - [Rule 9.5](#rule-95)
- [8.10 The Essential Type Model](#810-the-essential-type-model)
  - [Rule 10.1](#rule-101)
  - [Rule 10.2](#rule-102)
  - [Rule 10.3](#rule-103)
  - [Rule 10.4](#rule-104)
  - [Rule 10.5](#rule-105)
  - [Rule 10.6](#rule-106)
  - [Rule 10.7](#rule-107)
  - [Rule 10.8](#rule-108)
- [8.11 Pointer Type Conversions](#811-pointer-type-conversions)
  - [Rule 11.1](#rule-111)
  - [Rule 11.2](#rule-112)
  - [Rule 11.3](#rule-113)
  - [Rule 11.4](#rule-114)
  - [Rule 11.5](#rule-115)
  - [Rule 11.6](#rule-116)
  - [Rule 11.7](#rule-117)
  - [Rule 11.8](#rule-118)
  - [Rule 11.9](#rule-119)
- [8.12 Expressions](#812-expressions)
  - [Rule 12.1](#rule-121)
  - [Rule 12.2](#rule-122)
  - [Rule 12.3](#rule-123)
  - [Rule 12.4](#rule-124)
- [8.13 Side Effects](#813-side-effects)
  - [Rule 13.1](#rule-131)
  - [Rule 13.2](#rule-132)
  - [Rule 13.3](#rule-133)
  - [Rule 13.4](#rule-134)
  - [Rule 13.5](#rule-135)
  - [Rule 13.6](#rule-136)
- [8.14 Control Statement Expressions](#814-control-statement-expressions)
  - [Rule 14.1](#rule-141)
  - [Rule 14.2](#rule-142)
  - [Rule 14.3](#rule-143)
  - [Rule 14.4](#rule-144)
- [8.15 Control Flow](#815-control-flow)
  - [Rule 15.1](#rule-151)
  - [Rule 15.2](#rule-152)
  - [Rule 15.3](#rule-153)
  - [Rule 15.4](#rule-154)
  - [Rule 15.5](#rule-155)
  - [Rule 15.6](#rule-156)
  - [Rule 15.7](#rule-157)
- [8.16 Control Flow](#816-control-flow)
  - [Rule 16.1](#rule-161)
  - [Rule 16.2](#rule-162)
  - [Rule 16.3](#rule-163)
  - [Rule 16.4](#rule-164)
  - [Rule 16.5](#rule-165)
  - [Rule 16.6](#rule-166)
  - [Rule 16.7](#rule-167)
- [8.17 Functions](#817-functions)
  - [Rule 17.1](#rule-171)
  - [Rule 17.2](#rule-172)
  - [Rule 17.3](#rule-173)
  - [Rule 17.4](#rule-174)
  - [Rule 17.5](#rule-175)
  - [Rule 17.6](#rule-176)
  - [Rule 17.7](#rule-177)
  - [Rule 17.8](#rule-178)
- [8.18 Pointers and arrays](#818-pointers-and-arrays)
  - [Rule 18.1](#rule-181)
  - [Rule 18.2](#rule-182)
  - [Rule 18.3](#rule-183)
  - [Rule 18.4](#rule-184)
  - [Rule 18.5](#rule-185)
  - [Rule 18.6](#rule-186)
  - [Rule 18.7](#rule-187)
  - [Rule 18.8](#rule-188)
- [8.19 Overlapping storage](#819-overlapping-storage)
  - [Rule 19.1](#rule-191)
  - [Rule 19.2](#rule-192)
- [8.20 Overlapping storage](#820-overlapping-storage)
  - [Rule 20.1](#rule-201)
  - [Rule 20.2](#rule-202)
  - [Rule 20.3](#rule-203)
  - [Rule 20.4](#rule-204)
  - [Rule 20.5](#rule-205)
  - [Rule 20.6](#rule-206)
  - [Rule 20.7](#rule-207)
  - [Rule 20.8](#rule-208)
  - [Rule 20.9](#rule-209)
  - [Rule 20.10](#rule-2010)
  - [Rule 20.11](#rule-2011)
  - [Rule 20.12](#rule-2012)
  - [Rule 20.13](#rule-2013)
  - [Rule 20.14](#rule-2014)
- [8.21 Standard libraries](#821-standard-libraries)
  - [Rule 21.1](#rule-211)
  - [Rule 21.2](#rule-212)
  - [Rule 21.3](#rule-213)
  - [Rule 21.4](#rule-214)
  - [Rule 21.5](#rule-215)
  - [Rule 21.6](#rule-216)
  - [Rule 21.7](#rule-217)
  - [Rule 21.8](#rule-218)
  - [Rule 21.9](#rule-219)
  - [Rule 21.10](#rule-2110)
  - [Rule 21.11](#rule-2111)
  - [Rule 21.12](#rule-2112)
- [8.22 Resources](#822-resources)
  - [Rule 22.1](#rule-221)
  - [Rule 22.2](#rule-222)
  - [Rule 22.3](#rule-223)
  - [Rule 22.4](#rule-224)
  - [Rule 22.5](#rule-225)
  - [Rule 22.6](#rule-226)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


## 8.1 A Standard C Environment

### Rule 1.1

✔️ _No equivalent._

__[See Rule_1_1.rs](./tests/compile-fail/Rule_1_1.rs)__

### Rule 1.2

✔️ _No equivalent._

__[See Rule_1_2.rs](./tests/compile-fail/Rule_1_2.rs)__

### Rule 1.3

✔️ _Enforceable by default._

__[See Rule_1_3.rs](./tests/compile-fail/Rule_1_3.rs)__

## 8.2 Unused Code

### Rule 2.1

✔️ _Enforceable by default._

__[See Rule_2_1.rs](./tests/compile-fail/Rule_2_1.rs)__

### Rule 2.2

✔️ _Enforceable by default._

__[See Rule_2_2.rs](./tests/compile-fail/Rule_2_2.rs)__

### Rule 2.3

✔️ _Enforceable by default._

__[See Rule_2_3.rs](./tests/compile-fail/Rule_2_3.rs)__

### Rule 2.4

✔️ _Enforceable by default._

__[See Rule_2_4.rs](./tests/compile-fail/Rule_2_4.rs)__

### Rule 2.5

✔️ _Enforceable by default._

__[See Rule_2_5.rs](./tests/compile-fail/Rule_2_5.rs)__

### Rule 2.6

✔️ _Enforceable by default._

__[See Rule_2_6.rs](./tests/compile-fail/Rule_2_6.rs)__

### Rule 2.7

✔️ _Enforceable by default._

__[See Rule_2_7.rs](./tests/compile-fail/Rule_2_7.rs)__

## 8.3 Comments

### Rule 3.1

✖ **Not enforceable by default**

__[See Rule_3_1.rs](./tests/compile-fail/Rule_3_1.rs)__

### Rule 3.2

✔️ _Enforceable by default._

__[See Rule_3_2.rs](./tests/compile-fail/Rule_3_2.rs)__

## 8.4 Character sets and lexical conventions

### Rule 4.1

✔️ _Enforceable by default._

__[See Rule_4_1.rs](./tests/compile-fail/Rule_4_1.rs)__

### Rule 4.2

✔️ _No equivalent._

__[See Rule_4_2.rs](./tests/compile-fail/Rule_4_2.rs)__

## 8.5 Identifiers

### Rule 5.1

✖ **Not enforceable by default**

__[See Rule_5_1.rs](./tests/compile-fail/Rule_5_1.rs)__

### Rule 5.2

✖ **Not enforceable by default**

__[See Rule_5_2.rs](./tests/compile-fail/Rule_5_2.rs)__

### Rule 5.3

✖ **Not enforceable by default**

__[See Rule_5_3.rs](./tests/compile-fail/Rule_5_3.rs)__

### Rule 5.4

✖ **Not enforceable by default**

__[See Rule_5_4.rs](./tests/compile-fail/Rule_5_4.rs)__

### Rule 5.5

✔️ _Enforceable by default._

__[See Rule_5_5.rs](./tests/compile-fail/Rule_5_5.rs)__

### Rule 5.6

✖ **Not enforceable by default**

__[See Rule_5_6.rs](./tests/compile-fail/Rule_5_6.rs)__

### Rule 5.7

✖ **Not enforceable by default**

__[See Rule_5_7.rs](./tests/compile-fail/Rule_5_7.rs)__

### Rule 5.8

✔️ _Enforceable by default._

__[See Rule_5_8.rs](./tests/compile-fail/Rule_5_8.rs)__

### Rule 5.9

✔️ _Enforceable by default._

__[See Rule_5_9.rs](./tests/compile-fail/Rule_5_9.rs)__

## 8.6 Types

### Rule 6.1

✔️ _No equivalent._

__[See Rule_6_1.rs](./tests/compile-fail/Rule_6_1.rs)__

### Rule 6.2

✔️ _No equivalent._

__[See Rule_6_2.rs](./tests/compile-fail/Rule_6_2.rs)__

## 8.7 Literals and Constants

### Rule 7.1

✔️ _No equivalent._

__[See Rule_7_1.rs](./tests/compile-fail/Rule_7_1.rs)__

### Rule 7.2

✖ **Not enforceable by default**

__[See Rule_7_2.rs](./tests/compile-fail/Rule_7_2.rs)__

### Rule 7.3

✔️ _No equivalent._

__[See Rule_7_3.rs](./tests/compile-fail/Rule_7_3.rs)__

### Rule 7.4

✔️ _No equivalent._

__[See Rule_7_4.rs](./tests/compile-fail/Rule_7_4.rs)__

## 8.8 Literals and Constants

### Rule 8.1

✖ **Not enforceable by default**

__[See Rule_8_1.rs](./tests/compile-fail/Rule_8_1.rs)__

### Rule 8.2

✔️ _No equivalent._

__[See Rule_8_2.rs](./tests/compile-fail/Rule_8_2.rs)__

### Rule 8.3

✔️ _No equivalent._

__[See Rule_8_3.rs](./tests/compile-fail/Rule_8_3.rs)__

### Rule 8.4

✔️ _No equivalent._

__[See Rule_8_4.rs](./tests/compile-fail/Rule_8_4.rs)__

### Rule 8.5

✔️ _Enforceable by default._

__[See Rule_8_5.rs](./tests/compile-fail/Rule_8_5.rs)__

### Rule 8.6

✔️ _Enforceable by default._

__[See Rule_8_6.rs](./tests/compile-fail/Rule_8_6.rs)__

### Rule 8.7

✖ **Not enforceable by default**

__[See Rule_8_7.rs](./tests/compile-fail/Rule_8_7.rs)__

### Rule 8.8

✔️ _No equivalent._

__[See Rule_8_8.rs](./tests/compile-fail/Rule_8_8.rs)__

### Rule 8.9

✖ **Not enforceable by default**

__[See Rule_8_9.rs](./tests/compile-fail/Rule_8_9.rs)__

### Rule 8.10

✔️ _No equivalent._

__[See Rule_8_10.rs](./tests/compile-fail/Rule_8_10.rs)__

### Rule 8.11

✔️ _Enforceable by default._

__[See Rule_8_11.rs](./tests/compile-fail/Rule_8_11.rs)__

### Rule 8.12

✔️ _Enforceable by default._

__[See Rule_8_12.rs](./tests/compile-fail/Rule_8_12.rs)__

### Rule 8.13

✖ **Not enforceable by default**

__[See Rule_8_13.rs](./tests/compile-fail/Rule_8_13.rs)__

### Rule 8.14

✔️ _No equivalent._

__[See Rule_8_14.rs](./tests/compile-fail/Rule_8_14.rs)__

## 8.9 Initialization

### Rule 9.1

✔️ _Enforceable by default._

__[See Rule_9_1.rs](./tests/compile-fail/Rule_9_1.rs)__

### Rule 9.2

✔️ _Enforceable by default._

__[See Rule_9_2.rs](./tests/compile-fail/Rule_9_2.rs)__

### Rule 9.3

✔️ _Enforceable by default._

__[See Rule_9_3.rs](./tests/compile-fail/Rule_9_3.rs)__

### Rule 9.4

✔️ _Enforceable by default._

__[See Rule_9_4.rs](./tests/compile-fail/Rule_9_4.rs)__

### Rule 9.5

✔️ _Enforceable by default._

__[See Rule_9_5.rs](./tests/compile-fail/Rule_9_5.rs)__

## 8.10 The Essential Type Model

### Rule 10.1

✖ **Not enforceable by default**

__[See Rule_10_1.rs](./tests/compile-fail/Rule_10_1.rs)__

### Rule 10.2

✔️ _Enforceable by default._

__[See Rule_10_2.rs](./tests/compile-fail/Rule_10_2.rs)__

### Rule 10.3

✔️ _Enforceable by default._

__[See Rule_10_3.rs](./tests/compile-fail/Rule_10_3.rs)__

### Rule 10.4

✔️ _Enforceable by default._

__[See Rule_10_4.rs](./tests/compile-fail/Rule_10_4.rs)__

### Rule 10.5

✔️ _Enforceable by default._

__[See Rule_10_5.rs](./tests/compile-fail/Rule_10_5.rs)__

### Rule 10.6

✔️ _Enforceable by default._

__[See Rule_10_6.rs](./tests/compile-fail/Rule_10_6.rs)__

### Rule 10.7

✔️ _Enforceable by default._

__[See Rule_10_7.rs](./tests/compile-fail/Rule_10_7.rs)__

### Rule 10.8

✖ **Not enforceable by default**

__[See Rule_10_8.rs](./tests/compile-fail/Rule_10_8.rs)__

## 8.11 Pointer Type Conversions

### Rule 11.1

✔️ _Enforceable by default._

__[See Rule_11_1.rs](./tests/compile-fail/Rule_11_1.rs)__

### Rule 11.2

✔️ _Enforceable by default._

__[See Rule_11_2.rs](./tests/compile-fail/Rule_11_2.rs)__

### Rule 11.3

✔️ _Enforceable by default._

__[See Rule_11_3.rs](./tests/compile-fail/Rule_11_3.rs)__

### Rule 11.4

✔️ _Enforceable by default._

__[See Rule_11_4.rs](./tests/compile-fail/Rule_11_4.rs)__

### Rule 11.5

✔️ _No equivalent._

__[See Rule_11_5.rs](./tests/compile-fail/Rule_11_5.rs)__

### Rule 11.6

✔️ _No equivalent._

__[See Rule_11_6.rs](./tests/compile-fail/Rule_11_6.rs)__

### Rule 11.7

✔️ _Enforceable by default._

__[See Rule_11_7.rs](./tests/compile-fail/Rule_11_7.rs)__

### Rule 11.8

✔️ _Enforceable by default._

__[See Rule_11_8.rs](./tests/compile-fail/Rule_11_8.rs)__

### Rule 11.9

✔️ _No equivalent._

__[See Rule_11_9.rs](./tests/compile-fail/Rule_11_9.rs)__

## 8.12 Expressions

### Rule 12.1

✖ **Not enforceable by default**

__[See Rule_12_1.rs](./tests/compile-fail/Rule_12_1.rs)__

### Rule 12.2

✔️ _Enforceable by default._

__[See Rule_12_2.rs](./tests/compile-fail/Rule_12_2.rs)__

### Rule 12.3

✔️ _No equivalent._

__[See Rule_12_3.rs](./tests/compile-fail/Rule_12_3.rs)__

### Rule 12.4

✖ **Not enforceable by default**

__[See Rule_12_4.rs](./tests/compile-fail/Rule_12_4.rs)__

## 8.13 Side Effects

### Rule 13.1

✔️ _Enforceable by default._

__[See Rule_13_1.rs](./tests/compile-fail/Rule_13_1.rs)__

### Rule 13.2

✖ **Not enforceable by default**

__[See Rule_13_2.rs](./tests/compile-fail/Rule_13_2.rs)__

### Rule 13.3

✔️ _No equivalent._

__[See Rule_13_3.rs](./tests/compile-fail/Rule_13_3.rs)__

### Rule 13.4

✔️ _Enforceable by default._

__[See Rule_13_4.rs](./tests/compile-fail/Rule_13_4.rs)__

### Rule 13.5

✖ **Not enforceable by default**

__[See Rule_13_5.rs](./tests/compile-fail/Rule_13_5.rs)__

### Rule 13.6

✔️ _No equivalent._

__[See Rule_13_6.rs](./tests/compile-fail/Rule_13_6.rs)__

## 8.14 Control Statement Expressions

### Rule 14.1

✖ **Not enforceable by default**

__[See Rule_14_1.rs](./tests/compile-fail/Rule_14_1.rs)__

### Rule 14.2

✖ **Not enforceable by default**

__[See Rule_14_2.rs](./tests/compile-fail/Rule_14_2.rs)__

### Rule 14.3

✖ **Not enforceable by default**

__[See Rule_14_3.rs](./tests/compile-fail/Rule_14_3.rs)__

### Rule 14.4

✔️ _Enforceable by default._

__[See Rule_14_4.rs](./tests/compile-fail/Rule_14_4.rs)__

## 8.15 Control Flow

### Rule 15.1

✔️ _No equivalent._

__[See Rule_15_1.rs](./tests/compile-fail/Rule_15_1.rs)__

### Rule 15.2

✔️ _No equivalent._

__[See Rule_15_2.rs](./tests/compile-fail/Rule_15_2.rs)__

### Rule 15.3

✔️ _No equivalent._

__[See Rule_15_3.rs](./tests/compile-fail/Rule_15_3.rs)__

### Rule 15.4

✔️ _No equivalent._

__[See Rule_15_4.rs](./tests/compile-fail/Rule_15_4.rs)__

### Rule 15.5

✖ **Not enforceable by default**

__[See Rule_15_5.rs](./tests/compile-fail/Rule_15_5.rs)__

### Rule 15.6

✔️ _Enforceable by default._

__[See Rule_15_6.rs](./tests/compile-fail/Rule_15_6.rs)__

### Rule 15.7

✖ **Not enforceable by default**

__[See Rule_15_7.rs](./tests/compile-fail/Rule_15_7.rs)__

## 8.16 Control Flow

### Rule 16.1

✔️ _No equivalent._

__[See Rule_16_1.rs](./tests/compile-fail/Rule_16_1.rs)__

### Rule 16.2

✔️ _Enforceable by default._

__[See Rule_16_2.rs](./tests/compile-fail/Rule_16_2.rs)__

### Rule 16.3

✔️ _No equivalent._

__[See Rule_16_3.rs](./tests/compile-fail/Rule_16_3.rs)__

### Rule 16.4

✔️ _Enforceable by default._

__[See Rule_16_4.rs](./tests/compile-fail/Rule_16_4.rs)__

### Rule 16.5

✔️ _Enforceable by default._

__[See Rule_16_5.rs](./tests/compile-fail/Rule_16_5.rs)__

### Rule 16.6

✖ **Not enforceable by default**

__[See Rule_16_6.rs](./tests/compile-fail/Rule_16_6.rs)__

### Rule 16.7

✖ **Not enforceable by default**

__[See Rule_16_7.rs](./tests/compile-fail/Rule_16_7.rs)__

## 8.17 Functions

### Rule 17.1

✔️ _No equivalent._

__[See Rule_17_1.rs](./tests/compile-fail/Rule_17_1.rs)__

### Rule 17.2

✖ **Not enforceable by default**

__[See Rule_17_2.rs](./tests/compile-fail/Rule_17_2.rs)__

### Rule 17.3

✔️ _No equivalent._

__[See Rule_17_3.rs](./tests/compile-fail/Rule_17_3.rs)__

### Rule 17.4

✔️ _Enforceable by default._

__[See Rule_17_4.rs](./tests/compile-fail/Rule_17_4.rs)__

### Rule 17.5

✔️ _Enforceable by default._

__[See Rule_17_5.rs](./tests/compile-fail/Rule_17_5.rs)__

### Rule 17.6

✔️ _No equivalent._

__[See Rule_17_6.rs](./tests/compile-fail/Rule_17_6.rs)__

### Rule 17.7

✖ **Not enforceable by default**

__[See Rule_17_7.rs](./tests/compile-fail/Rule_17_7.rs)__

### Rule 17.8

__[See Rule_17_8.rs](./tests/compile-fail/Rule_17_8.rs)__

✖ **Not enforceable by default**

## 8.18 Pointers and arrays

### Rule 18.1

✔️ _Enforceable by default._

__[See Rule_18_1.rs](./tests/compile-fail/Rule_18_1.rs)__

### Rule 18.2

✔️ _Enforceable by default._

__[See Rule_18_2.rs](./tests/compile-fail/Rule_18_2.rs)__

### Rule 18.3

✔️ _No equivalent._

__[See Rule_18_3.rs](./tests/compile-fail/Rule_18_3.rs)__

### Rule 18.4

✔️ _Enforceable by default._

__[See Rule_18_4.rs](./tests/compile-fail/Rule_18_4.rs)__

### Rule 18.5

✖ **Not enforceable by default**

__[See Rule_18_5.rs](./tests/compile-fail/Rule_18_5.rs)__

### Rule 18.6

✔️ _Enforceable by default._

__[See Rule_18_6.rs](./tests/compile-fail/Rule_18_6.rs)__

### Rule 18.7

✔️ _Enforceable by default._

__[See Rule_18_7.rs](./tests/compile-fail/Rule_18_7.rs)__

### Rule 18.8

✔️ _Enforceable by default._

__[See Rule_18_8.rs](./tests/compile-fail/Rule_18_8.rs)__

## 8.19 Overlapping storage

### Rule 19.1

✔️ _Enforceable by default._

__[See Rule_19_1.rs](./tests/compile-fail/Rule_19_1.rs)__

### Rule 19.2

✖ **Not enforceable by default**

__[See Rule_19_2.rs](./tests/compile-fail/Rule_19_1.rs)__

## 8.20 Overlapping storage

### Rule 20.1

✖ **Not enforceable by default**

__[See Rule_20_1.rs](./tests/compile-fail/Rule_20_1.rs)__

### Rule 20.2

✖ **Not enforceable by default**

__[See Rule_20_2.rs](./tests/compile-fail/Rule_20_2.rs)__

### Rule 20.3

✔️ _No equivalent._

__[See Rule_20_3.rs](./tests/compile-fail/Rule_20_3.rs)__

### Rule 20.4

✔️ _Enforceable by default._

__[See Rule_20_4.rs](./tests/compile-fail/Rule_20_4.rs)__

### Rule 20.5

✔️ _No equivalent._

__[See Rule_20_5.rs](./tests/compile-fail/Rule_20_5.rs)__

### Rule 20.6

✔️ _Enforceable by default._

__[See Rule_20_6.rs](./tests/compile-fail/Rule_20_6.rs)__

### Rule 20.7

✔️ _Enforceable by default._

__[See Rule_20_7.rs](./tests/compile-fail/Rule_20_7.rs)__

### Rule 20.8

✔️ _No equivalent._

__[See Rule_20_8.rs](./tests/compile-fail/Rule_20_8.rs)__

### Rule 20.9

✔️ _No equivalent._

__[See Rule_20_9.rs](./tests/compile-fail/Rule_20_9.rs)__

### Rule 20.10

✔️ _No equivalent._

__[See Rule_20_10.rs](./tests/compile-fail/Rule_20_10.rs)__

### Rule 20.11

✔️ _No equivalent._

__[See Rule_20_11.rs](./tests/compile-fail/Rule_20_11.rs)__

### Rule 20.12

✔️ _No equivalent._

__[See Rule_20_12.rs](./tests/compile-fail/Rule_20_12.rs)__

### Rule 20.13

✔️ _No equivalent._

__[See Rule_20_13.rs](./tests/compile-fail/Rule_20_13.rs)__

### Rule 20.14

✔️ _No equivalent._

__[See Rule_20_14.rs](./tests/compile-fail/Rule_20_14.rs)__

## 8.21 Standard libraries

### Rule 21.1

✖ **Not enforceable by default**

__[See Rule_21_1.rs](./tests/compile-fail/Rule_21_1.rs)__

### Rule 21.2

✖ **Not enforceable by default**

__[See Rule_21_2.rs](./tests/compile-fail/Rule_21_2.rs)__

### Rule 21.3

✔️ _No equivalent._

__[See Rule_21_3.rs](./tests/compile-fail/Rule_21_3.rs)__

### Rule 21.4

✔️ _No equivalent._

__[See Rule_21_4.rs](./tests/compile-fail/Rule_21_4.rs)__

### Rule 21.5

✔️ _No equivalent._

__[See Rule_21_5.rs](./tests/compile-fail/Rule_21_5.rs)__

### Rule 21.6

✔️ _No equivalent._

__[See Rule_21_6.rs](./tests/compile-fail/Rule_21_6.rs)__

### Rule 21.7

✔️ _No equivalent._

__[See Rule_21_7.rs](./tests/compile-fail/Rule_21_7.rs)__

### Rule 21.8

✔️ _No equivalent._

__[See Rule_21_8.rs](./tests/compile-fail/Rule_21_8.rs)__

### Rule 21.9

✔️ _No equivalent._

__[See Rule_21_9.rs](./tests/compile-fail/Rule_21_9.rs)__

### Rule 21.10

✔️ _No equivalent._

__[See Rule_21_10.rs](./tests/compile-fail/Rule_21_10.rs)__

### Rule 21.11

✔️ _No equivalent._

__[See Rule_21_11.rs](./tests/compile-fail/Rule_21_11.rs)__

### Rule 21.12

✔️ _No equivalent._

__[See Rule_21_12.rs](./tests/compile-fail/Rule_21_12.rs)__

## 8.22 Resources

### Rule 22.1

✔️ _No equivalent._

__[See Rule_22_1.rs](./tests/compile-fail/Rule_22_1.rs)__

### Rule 22.2

✔️ _No equivalent._

__[See Rule_22_2.rs](./tests/compile-fail/Rule_22_2.rs)__

### Rule 22.3

✔️ _No equivalent._

__[See Rule_22_3.rs](./tests/compile-fail/Rule_22_3.rs)__

### Rule 22.4

✔️ _No equivalent._

__[See Rule_22_4.rs](./tests/compile-fail/Rule_22_4.rs)__

### Rule 22.5

✔️ _No equivalent._

__[See Rule_22_5.rs](./tests/compile-fail/Rule_22_5.rs)__

### Rule 22.6

✔️ _No equivalent._

__[See Rule_22_6.rs](./tests/compile-fail/Rule_22_6.rs)__
