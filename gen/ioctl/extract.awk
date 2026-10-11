function hex(value, result, quotient, remainder, digit, i) {
    result = ""
    do {
        quotient = ""
        remainder = 0
        for (i = 1; i <= length(value); i++) {
            digit = remainder * 10 + substr(value, i, 1)
            if (quotient != "" || digit >= 16) {
                quotient = quotient sprintf("%d", int(digit / 16))
            }
            remainder = digit % 16
        }
        result = substr("0123456789abcdef", remainder + 1, 1) result
        value = quotient
    } while (value != "")
    return "0x" result
}

function fail(message) {
    print "ioctl extraction: " message > "/dev/stderr"
    failed = 1
    exit 1
}

BEGIN {
    printf "%s", "" > names
    print condition
}

/EnumConstantDecl/ && match($0, /ioctl_value_[[:alnum:]_]+/) {
    if (name != "") {
        fail("missing value for " name)
    }
    name = substr($0, RSTART + 12, RLENGTH - 12)
}

name != "" && /value: Int / {
    value = $NF
    if (value !~ /^[0-9]+$/) {
        fail("invalid unsigned value for " name)
    }
    printf "#define %s %sul\n", name, hex(value)
    print name > names
    name = ""
    count++
}

END {
    if (failed) {
        exit 1
    }
    if (name != "") {
        fail("missing value for " name)
    }
    if (!count) {
        fail("Clang produced no ioctl constants")
    }
    print "#endif"
}
