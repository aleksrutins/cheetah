#include <cmark.h>
#include <stdio.h>
#include <stdlib.h>
#include <linux/limits.h>
#include <string.h>
#include <dirent.h>
#define BUILD_DIR "_build"
void copy_streaming(FILE *in, FILE *out) {
	int c;
	while ((c = fgetc(in)) != EOF) {
		fputc(c, out);
	}
}

int has_extension(const char *filename, const char *ext) {
	char *pExt = strchr(filename, '.');
	return !strcmp(pExt, ext);
}

void convert_page(char *filename, FILE *header, FILE *footer) {
	char buffer[256];
	char full_path[PATH_MAX];
	strcpy(full_path, "pages");
	strcat(full_path, filename);
	FILE *fp = fopen(full_path, "rb");

	if(fp == NULL) {
		perror("Failed to open file");
		return;
	}

	cmark_parser *parser = cmark_parser_new(CMARK_OPT_DEFAULT);

	int bytes;
	while ((bytes = fread(buffer, 1, sizeof(buffer), fp)) > 0) {
		cmark_parser_feed(parser, buffer, bytes);
		if(bytes < sizeof(buffer)) {
			break;
		}
	}

	cmark_node *document = cmark_parser_finish(parser);
	cmark_parser_free(parser);
	fclose(fp);
	
	// Write to built page
	char outpath[PATH_MAX];
	strcpy(outpath, BUILD_DIR "/");
	strcat(outpath, filename);

	FILE *outfp = fopen(outpath, "w");
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
	while ((dp = readdir(dir)) != NULL) {
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
	
	compile_templates_recursively(header, footer, "pages");

	fclose(header);
	fclose(footer);
	return 0;
}
