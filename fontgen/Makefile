fontgen: fontgen.rs libstb_image.a
	rustc -L. fontgen.rs

libstb_image.a: stb_image.o
	ar -crs libstb_image.a stb_image.o

stb_image.o: stb_image.h
	cc -x c -DSTB_IMAGE_IMPLEMENTATION -o stb_image.o -c stb_image.h
