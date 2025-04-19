INCLUDEDIR=$1
LIBDIR=$2

cat <<EOF
%rename cpp_options old_cpp_options

*cpp_options:
-nostdinc -isystem $INCLUDEDIR %(old_cpp_options)

*cc1:
-D__aarch64__ -nostdinc -isystem $INCLUDEDIR

*lib:
-lrlibc -lrlibc_helper

*startfile:
%s

*endfile:
%s

*link:
-static -nostdlib -L$LIBDIR -lrlibc -lrlibc_helper

*startfile_prefix_spec:

*cc1plus: