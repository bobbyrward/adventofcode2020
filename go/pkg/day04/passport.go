package day04

import (
	"strconv"
	"strings"
)

const (
	FIELD_BYR string = "byr"
	FIELD_IYR string = "iyr"
	FIELD_EYR string = "eyr"
	FIELD_HGT string = "hgt"
	FIELD_HCL string = "hcl"
	FIELD_ECL string = "ecl"
	FIELD_PID string = "pid"
)

/*
byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:

    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.

hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
*/
func splitPassportField(field string) []string {
	return strings.Split(field, ":")
}

type Passport struct {
	fields map[string]string
}

func NewPassport(input string) *Passport {
	fields := make(map[string]string)

	for _, field := range strings.Fields(input) {
		parts := splitPassportField(field)

		fields[parts[0]] = parts[1]
	}

	return &Passport{
		fields: fields,
	}
}

func (passport *Passport) HasAllFields(required_fields []string) bool {
	for _, required := range required_fields {
		_, exists := passport.fields[required]

		if !exists {
			return false
		}
	}

	return true

}

func isNumberBetween(value string, min int, max int) bool {
	byr, err := strconv.Atoi(value)
	if err != nil {
		return false
	}

	if byr < min || byr > max {
		return false
	}

	return true
}

func validateHeight(hgt string) bool {
	hgt_len := len(hgt)

	height, err := strconv.Atoi(hgt[:hgt_len-2])
	if err != nil {
		return false
	}

	unit := hgt[hgt_len-2:]
	var min, max int

	if unit == "cm" {
		min = 150
		max = 193
	} else if unit == "in" {
		min = 59
		max = 76
	} else {
		return false
	}

	if height < min || height > max {
		return false
	}

	return true
}

func validateHairColor(hcl string) bool {
	hcl_len := len(hcl)

	if hcl_len != 7 {
		return false
	}

	if hcl[:1] != "#" {
		return false
	}

	_, err := strconv.ParseUint(hcl[1:], 16, 32)
	if err != nil {
		return false
	}

	return true
}

func validateEyeColor(ecl string) bool {
	eye_colors := []string{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}

	for _, valid := range eye_colors {
		if valid == ecl {
			return true
		}
	}

	return false
}

func validatePassportId(pid string) bool {
	if len(pid) != 9 {
		return false
	}

	_, err := strconv.Atoi(pid)
	if err != nil {
		return false
	}

	return true
}

func (passport *Passport) Validate() bool {
	if !isNumberBetween(passport.fields[FIELD_BYR], 1920, 2002) {
		return false
	}

	if !isNumberBetween(passport.fields[FIELD_IYR], 2010, 2020) {
		return false
	}

	if !isNumberBetween(passport.fields[FIELD_EYR], 2020, 2030) {
		return false
	}

	if !validateHeight(passport.fields[FIELD_HGT]) {
		return false
	}

	if !validateHairColor(passport.fields[FIELD_HCL]) {
		return false
	}

	if !validateEyeColor(passport.fields[FIELD_ECL]) {
		return false
	}

	if !validateEyeColor(passport.fields[FIELD_ECL]) {
		return false
	}

	if !validatePassportId(passport.fields[FIELD_PID]) {
		return false
	}

	return true
}
