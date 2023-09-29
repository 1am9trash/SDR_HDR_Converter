# HDR SDR converter

## Basic Information
A lightweight converter between different formats of images.

## Run
Use `cargo run` to execute the sample code.

## Features
For an image, it has below formats.

| format | intro |
| --- | --- |
| `dynamic range` | The brightness range of the display signal, determining whether it's in HDR or SDR format. |
| `signal status` | The state of the signal, which can be optical or electrical, transformed using different EOTFs/OETFs. |
| `color depth` | The number of bits used by each color channel. |
| `color space` | Determining how colors are represented and the gamut coverage. |
| `color format` | Specifying the arrangement of color channels within a pixel. |

This work makes it easy to convert images with arbitrary formats. Currently supported formats are:

| format | supported classes |
| --- | --- |
| `dynamic range` | `HDR`, `SDR` |
| `signal status` | `Linear`, `PQ`, `HLG`, `Gamma709` |
| `color depth` | `8-bit`, `16-bit` |
| `color space` | `Rec709`, `Rec2020` |
| `color format` | `RGB` |

## Procedure

1. Nonrmalizing the input to `[0, 1]`
2. EOTF transformation
3. Color space conversion
4. Tone mapping
5. OETF transformation
6. Quantization to correct range based on color depth

## Performance

| Original HDR | To SDR | Back To HDR |
| --- | --- | --- |
| <img src="sample/origin_field_hdr_linear_16.png" width = "300" height = "203" align=center /> | <img src="sample/output_field_sdr_709_8.png" width = "300" height = "203" align=center /> | <img src="sample/output_field_hdr_linear_16.png" width = "300" height = "203" align=center /> |
| <img src="sample/origin_cloud_hdr_linear_8.png" width = "300" height = "125" align=center /> | <img src="sample/output_cloud_sdr_709_8.png" width = "300" height = "125" align=center /> | <img src="sample/output_cloud_hdr_linear_8.png" width = "300" height = "125" align=center /> |

## Todo
- Create more friendly interface, maybe turn the project into a command line tool.
- Extend TMO and iTMO.
- Currently PQ, HLG seens a little bit too bright. Take some time to check it.

## References
- [Rec. ITU-R BT.2087-0](https://www.itu.int/dms_pubrec/itu-r/rec/bt/R-REC-BT.2087-0-201510-I!!PDF-E.pdf): for color conversion.
- [REPORT ITU-R BT.2446-1](https://www.itu.int/dms_pub/itu-r/opb/rep/R-REP-BT.2446-1-2021-PDF-E.pdf): for untrival TMO and untrival iTMO.
- [MovieLabs Best Practices for Mapping SDR to HDR10](https://www.movielabs.com/ngvideo/MovieLabs_Mapping_BT.709_to_HDR10_v1.0.pdf): for trival iTMO.
