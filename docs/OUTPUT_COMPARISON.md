# Output Format Comparison 📑

This document compares the output of `dex-parser-rust` against traditional Android SDK tools.

## 🏁 Summary Table

| Feature | dexdump (Official) | baksmali | dex-parser-rust |
| :--- | :--- | :--- | :--- |
| **Output Format** | Plain Text / XML | Smali Files | Clean Text / **JSON** |
| **Operand Resolution** | Indexed (`string@001a`) | Fully Resolved | **Fully Resolved** |
| **Branching** | Relative Offsets | Labels | **Physical Labels (`:label_`)** |
| **Debug Info** | Separate Section | Integrated | **Integrated In-line** |
| **Access Flags** | Hex + Text | Text Only | **Standard Pipe (`PUBLIC \| STATIC`)** |

---

## 🔍 Disassembly Comparison

### Official `dexdump` (Standard)
`dexdump` output is often cluttered and requires manual effort to resolve indices.
```text
0002b8: 1a01 0d00      |0000: const-string v1, string@000d // "AES/ECB/PKCS7Padding"
0002bc: 7020 2400 2001 |0004: invoke-direct {v0, v2, v1}, meth@0024 // SecretKeySpec.<init>
```

### `dex-parser-rust` (Our Output)
Our output is designed for **Human Auditing**. It removes the "noise" and provides resolved context immediately.
```smali
0x0000: const-string v1, "AES/ECB/PKCS7Padding"
0x0004: invoke-direct {v0, v2, v1}, Ljavax/crypto/spec/SecretKeySpec;-><init>([BLjava/lang/String;)V
```
*   **Resolved Signatures**: Full class and method signatures are shown, not just names.
*   **Clean Strings**: Quotes are automatically added, and the `string@` prefix is removed for clarity.

---

## 🛣️ Logic & Control Flow

One of the biggest pain points in `dexdump` is following jumps.

### `dexdump` Jumps
You have to manually calculate where `+0008` lands.
```text
001e: if-ge v3, v1, +0008 // target 0026
...
0026: return v2
```

### `dex-parser-rust` Labels
We use **Physical Labeling** (inspired by `baksmali`) to make CFG analysis instant.
```smali
      :label_001e
0x001e: if-ge v3, v1, :label_0026
...
      :label_0026
0x0026: return v2
```

---

## 🛠️ Integrated Debug Information

We interleave `.line` and `.local` information directly into the disassembly, whereas other tools often put them in a separate, disconnected section.

```smali
      .line 42
      .local "password":Ljava/lang/String;
0x0010: iget-object v0, v0, Lcom/app/User;->token:Ljava/lang/String;
```

---

## 📊 Machine-Readable JSON

Unlike its competitors, `dex-parser-rust` offers a first-class **JSON Export**. This allows developers to build their own tools (GUIs, Emulators, Static Analyzers) without writing a complex parser from scratch.

```json
{
  "name": "new-instance",
  "description": "v0, Ljavax/crypto/spec/SecretKeySpec;",
  "index": 28,
  "resolved_value": "Ljavax/crypto/spec/SecretKeySpec;"
}
```
