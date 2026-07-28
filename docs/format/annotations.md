# DEX Format: Annotations

## annotations_directory_item
| Name | Format | Description |
| :--- | :--- | :--- |
| class_annotations_off | uint | offset to `annotation_set_item` |
| fields_size | uint | count of annotated fields |
| annotated_methods_size | uint | count of annotated methods |
| annotated_parameters_size | uint | count of annotated parameters |
| field_annotations | `field_annotation[]` | |
| method_annotations | `method_annotation[]` | |
| parameter_annotations | `parameter_annotation[]` | |

## annotation_set_item
- `size`: uint
- `entries`: `annotation_off_item[size]`

### annotation_off_item
- `annotation_off`: uint - offset to `annotation_item`.

## annotation_item
| Name | Format | Description |
| :--- | :--- | :--- |
| visibility | ubyte | `BUILD`, `RUNTIME`, or `SYSTEM` |
| annotation | `encoded_annotation` | encoded annotation contents |

### Visibility values
- `VISIBILITY_BUILD` (0x00)
- `VISIBILITY_RUNTIME` (0x01)
- `VISIBILITY_SYSTEM` (0x02)
