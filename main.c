#include <cmark.h>
#include <stdio.h>
#include <stdlib.h>
#if (defined(LINUX) || defined(__linux__))
#	include <linux/limits.h>
#else
#	include <sys/syslimits.h>
#endif
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>

#define BUILD_DIR "_build"

int strchr_last(const char *str, char c) {
	int len = strlen(str);
	for(int i = len-1; i >= 0; i--) {
		if(str[i] == c) return i;
	}
	return -1;
}

void copy_streaming(FILE *in, FILE *out) {
	int c;
	while ((c = fgetc(in)) != EOF) {
		fputc(c, out);
	}
	rewind(in);
}

int has_extension(const char *filename, const char *ext) {
	int extpos = strchr_last(filename, '.');
	if(extpos == -1) return 0;
	return !strcmp(filename + extpos, ext);
}

void set_extension(char *filename, const char *ext) {
	int extpos = strchr_last(filename, '.');
       	if(extpos != -1) {
		*(filename + extpos) = 0;
	}
	strcat(filename, ext);
}

void convert_page(char *filename, FILE *header, FILE *footer) {
	FILE *fp = fopen(filename, "rb");

	if(fp == NULL) {
		perror("Failed to open file");
		return;
	}

	cmark_node *document = cmark_parse_file(fp, CMARK_OPT_DEFAULT);
	fclose(fp);
	
	// Write to built page
	char outpath[PATH_MAX];
	strcpy(outpath, BUILD_DIR "/");
	strcat(outpath, filename);
	set_extension(outpath, ".html");

	FILE *outfp = fopen(outpath, "w");
	if(outfp == NULL) {
		perror("Failed to open output file");
		return;
	}
	char *rendered = cmark_render_html(document, CMARK_OPT_DEFAULT);
	
	if(header != NULL) {
		copy_streaming(header, outfp);
	}
	fputs(rendered, outfp);
	if(footer != NULL) {
		copy_streaming(footer, outfp);
	}

	fclose(outfp);
	free(rendered);
	cmark_node_free(document);
}

void compile_templates_recursively(FILE *header, FILE *footer, char *path) {
	struct dirent *dp;
	DIR *dir = opendir(path);
	if(dir == NULL) {
		printf("warning: template directory `%s` not found.\n", path);
		return;
	}
	char out_path[PATH_MAX];
	strcpy(out_path, BUILD_DIR "/");
	strcat(out_path, path);
	mkdir(out_path, S_IRWXU);
	while ((dp = readdir(dir)) != NULL) if(dp->d_name[0] != '.') {
		char file_path[PATH_MAX];
		strcpy(file_path, path);
		strcat(file_path, "/");
		strcat(file_path, dp->d_name);
		if(dp->d_type == DT_DIR) {
			compile_templates_recursively(header, footer, file_path);
		} else if(has_extension(dp->d_name, ".md")) {
			convert_page(file_path, header, footer);
		} else if(has_extension(dp->d_name, ".html")) {
			char outpath[PATH_MAX];
			strcpy(outpath, BUILD_DIR "/");
			strcat(outpath, file_path);
			FILE *out_fp = fopen(outpath, "w");
			FILE *src_fp = fopen(file_path, "r");
			copy_streaming(header, out_fp);
			copy_streaming(src_fp, out_fp);
			copy_streaming(footer, out_fp);
			fclose(out_fp);
			fclose(src_fp);
		}
	}
}

int main() {
	FILE *header = fopen("layout/header.html", "r");
	FILE *footer = fopen("layout/footer.html", "r");
	
	mkdir("_build", S_IRWXU);
	compile_templates_recursively(header, footer, "pages");

	fclose(header);
	fclose(footer);
	return 0;
}
