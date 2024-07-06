package pencoder

import (
	"bytes"
	"testing"
)

func TestHexEncoder_Encode(t *testing.T) {
	tests := []struct {
		in  []byte
		out string
	}{
		{[]byte{0, 0, 0}, "000000"},
		{[]byte{1, 1, 1}, "010101"},
		{[]byte{9, 9, 9}, "090909"},
		{[]byte{10, 255, 255}, "0affff"},
	}

	buf := make([]byte, 255)
	for _, test := range tests {
		var encoder HexEncoder
		n := encoder.Encode(buf, test.in)
		encoded := buf[:n]
		if string(encoded) != test.out {
			t.Errorf("Expected %s, got %s", test.out, string(encoded))
		}
	}
}

func TestHexEncoder_UpperAlpha(t *testing.T) {
	tests := []struct {
		in  []byte
		out string
	}{
		{[]byte{10, 255, 255}, "0AFFFF"},
	}

	buf := make([]byte, 255)
	for _, test := range tests {
		encoder := HexEncoder{
			Mode: HexModeUpper,
		}
		n := encoder.Encode(buf, test.in)
		encoded := buf[:n]
		if string(encoded) != test.out {
			t.Errorf("Expected %s, got %s", test.out, string(encoded))
		}
	}
}

func TestHexEncoder_Decode(t *testing.T) {
	tests := []struct {
		in  string
		out []byte
	}{
		{"000000", []byte{0, 0, 0}},
		{"010101", []byte{1, 1, 1}},
		{"090909", []byte{9, 9, 9}},
		{"0affff", []byte{10, 255, 255}},
	}

	buf := make([]byte, 255)
	for _, test := range tests {
		var encoder HexEncoder
		n := encoder.Decode(buf, []byte(test.in))
		encoded := buf[:n]
		if !bytes.Equal(encoded, test.out) {
			t.Errorf("Expected %#v, got %#v", test.out, encoded)
		}
	}
}
