# Pseudo-Instructions (Payloads)

These are not true opcodes but rather data payloads referenced by other instructions. They always start on an even (2-byte) boundary.

| Magic | Name | Description |
|-------|------|-------------|
| 0x0100 | packed-switch-payload | Data for a `packed-switch` instruction. |
| 0x0200 | sparse-switch-payload | Data for a `sparse-switch` instruction. |
| 0x0300 | fill-array-data-payload | Data for a `fill-array-data` instruction. |

## Details

### packed-switch-payload
- `size`: number of entries
- `first_key`: the first (smallest) key in the switch
- `targets`: list of relative branch targets

### sparse-switch-payload
- `size`: number of entries
- `keys`: sorted list of keys
- `targets`: list of relative branch targets (matching the keys)

### fill-array-data-payload
- `element_width`: size of each element in bytes
- `size`: number of elements
- `data`: raw bytes of the array data
