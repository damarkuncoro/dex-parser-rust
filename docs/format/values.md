# DEX Format: Encoded Values

## encoded_value
An `encoded_value` is a compact representation of arbitrary hierarchically structured data.
- Header byte: `(value_arg << 5) | value_type`
- Value: `ubyte[]`

### Value Types
- `VALUE_BYTE` (0x00)
- `VALUE_SHORT` (0x02)
- `VALUE_INT` (0x04)
- `VALUE_LONG` (0x06)
- `VALUE_STRING` (0x17)
- `VALUE_TYPE` (0x18)
- `VALUE_FIELD` (0x19)
- `VALUE_METHOD` (0x1a)
- `VALUE_ENUM` (0x1b)
- `VALUE_ARRAY` (0x1c)
- `VALUE_ANNOTATION` (0x1d)
- `VALUE_NULL` (0x1e)
- `VALUE_BOOLEAN` (0x1f)

## encoded_array
- `size`: uleb128
- `values`: `encoded_value[size]`

## call_site_id_item
- `call_site_off`: uint - offset to `call_site_item`.

## method_handle_item
- `method_handle_type`: ushort
- `field_or_method_id`: ushort
