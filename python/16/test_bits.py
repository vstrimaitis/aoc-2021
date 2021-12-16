import pytest
from bits import *


@pytest.mark.parametrize(
    "hex_input,zeros_left,expected_packet",
    [
        ("D2FE28", 3, Literal(version=6, value=2021)),
        (
            "38006F45291200",
            7,
            LessThan(
                version=1,
                children=[
                    Literal(version=6, value=10),
                    Literal(version=2, value=20),
                ],
            ),
        ),
        (
            "EE00D40C823060",
            5,
            Maximum(
                version=7,
                children=[
                    Literal(version=2, value=1),
                    Literal(version=4, value=2),
                    Literal(version=1, value=3),
                ],
            ),
        ),
        (
            "8A004A801A8002F478",
            3,
            Minimum(
                version=4,
                children=[
                    Minimum(
                        version=1,
                        children=[
                            Minimum(version=5, children=[Literal(version=6, value=15)])
                        ],
                    )
                ],
            ),
        ),
        (
            "620080001611562C8802118E34",
            2,
            Sum(
                version=3,
                children=[
                    Sum(
                        version=0,
                        children=[
                            Literal(version=0, value=10),
                            Literal(version=5, value=11),
                        ],
                    ),
                    Sum(
                        version=1,
                        children=[
                            Literal(version=0, value=12),
                            Literal(version=3, value=13),
                        ],
                    ),
                ],
            ),
        ),
        (
            "C0015000016115A2E0802F182340",
            6,
            Sum(
                version=6,
                children=[
                    Sum(
                        version=0,
                        children=[
                            Literal(version=0, value=10),
                            Literal(version=6, value=11),
                        ],
                    ),
                    Sum(
                        version=4,
                        children=[
                            Literal(version=7, value=12),
                            Literal(version=0, value=13),
                        ],
                    ),
                ],
            ),
        ),
        (
            "A0016C880162017C3686B18A3D4780",
            7,
            Sum(
                version=5,
                children=[
                    Sum(
                        version=1,
                        children=[
                            Sum(
                                version=3,
                                children=[
                                    Literal(version=7, value=6),
                                    Literal(version=6, value=6),
                                    Literal(version=5, value=12),
                                    Literal(version=2, value=15),
                                    Literal(version=2, value=15),
                                ],
                            )
                        ],
                    )
                ],
            ),
        ),
    ],
)
def test_parse(hex_input, zeros_left, expected_packet):
    p, s = parse_hex(hex_input)
    assert len(s) == zeros_left
    assert str(p) == str(expected_packet)


@pytest.mark.parametrize(
    "hex_input,expected_value",
    [
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
        (
            "220D790065B2745FF004672D99A34E5B33439D96CEC80373C0068663101A98C406A5E7395DC1804678BF25A4093BFBDB886CA6E11FDE6D93D16A100325E5597A118F6640600ACF7274E6A5829B00526C167F9C089F15973C4002AA4B22E800FDCFD72B9351359601300424B8C9A00BCBC8EE069802D2D0B945002AB2D7D583E3F00016B05E0E9802BA00B4F29CD4E961491CCB44C6008E80273C393C333F92020134B003530004221347F83A200D47F89913A66FB6620016E24A007853BE5E944297AB64E66D6669FCEA0112AE06009CAA57006A0200EC258FB0440010A8A716A321009DE200D44C8E31F00010887B146188803317A3FC5F30056C0150004321244E88C000874468A91D2291802B25EB875802B28D13550030056C0169FB5B7ECE2C6B2EF3296D6FD5F54858015B8D730BB24E32569049009BF801980803B05A3B41F1007625C1C821256D7C848025DE0040E5016717247E18001BAC37930E9FA6AE3B358B5D4A7A6EA200D4E463EA364EDE9F852FF1B9C8731869300BE684649F6446E584E61DE61CD4021998DB4C334E72B78BA49C126722B4E009C6295F879002093EF32A64C018ECDFAF605989D4BA7B396D9B0C200C9F0017C98C72FD2C8932B7EE0EA6ADB0F1006C8010E89B15A2A90021713610C202004263E46D82AC06498017C6E007901542C04F9A0128880449A8014403AA38014C030B08012C0269A8018E007A801620058003C64009810010722EC8010ECFFF9AAC32373F6583007A48CA587E55367227A40118C2AC004AE79FE77E28C007F4E42500D10096779D728EB1066B57F698C802139708B004A5C5E5C44C01698D490E800B584F09C8049593A6C66C017100721647E8E0200CC6985F11E634EA6008CB207002593785497652008065992443E7872714",
            7936430475134,
        ),
    ],
)
def test_evaluate(hex_input, expected_value):
    p, _ = parse_hex(hex_input)
    assert p.evaluate() == expected_value
