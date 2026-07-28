# DEX Format: System Annotations

System annotations represent reflective information and have `VISIBILITY_SYSTEM`.

## dalvik.annotation.AnnotationDefault
Attached to each annotation interface to indicate default bindings.

## dalvik.annotation.EnclosingClass
Attached to each class defined as a member of another class or anonymous (but not in a method).

## dalvik.annotation.EnclosingMethod
Attached to each class defined inside a method body.

## dalvik.annotation.InnerClass
Attached to each class defined in the lexical scope of another class.

## dalvik.annotation.MemberClasses
Attached to each class that declares member classes.

## dalvik.annotation.MethodParameters
Provides parameter metadata like names and modifiers. (Android 7.1+).

## dalvik.annotation.Signature
Attached to classes, fields, or methods defined in terms of more complicated types than `type_id_item` can represent.

## dalvik.annotation.Throws
Attached to methods declared to throw exceptions.
