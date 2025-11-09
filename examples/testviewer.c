#include <gtk/gtk.h>
#include "gtkmdviewer.h"

typedef struct {
    char *file_path;
} AppData;

static void activate(GtkApplication *app, gpointer user_data) {
    AppData *app_data = (AppData*)user_data;

    GtkWidget *window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Markdown Editor Test");
    gtk_window_set_default_size(GTK_WINDOW(window), 600, 400);

    // Read content from file
    GError *error = NULL;
    g_autofree gchar *content;
    if (!g_file_get_contents(app_data->file_path, &content, NULL, &error)) {
        g_printerr("Error reading file '%s': %s\n", app_data->file_path, error->message);
        g_error_free(error);
        g_application_quit(G_APPLICATION(app));
        return;
    }
    g_autofree const char* img_prefix = g_path_get_dirname(app_data->file_path);

    // Create a scrolled window
    GtkWidget *scrolled = gtk_scrolled_window_new();
    gtk_window_set_child(GTK_WINDOW(window), scrolled);

    // Add the editor to the scrolled window
    GtkWidget *editor = gtk_md_viewer_new_with_img_prefix(content, img_prefix);

    gtk_scrolled_window_set_child(GTK_SCROLLED_WINDOW(scrolled), editor);

    gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char **argv) {
    AppData app_data;

    // Use provided file path or default to examples/test.md
    if (argc > 1) {
        app_data.file_path = argv[1];
    } else {
        app_data.file_path = "examples/test.md";
    }

    GtkApplication *app = gtk_application_new("org.gtk.example", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK(activate), &app_data);
    int status = g_application_run(G_APPLICATION(app), argc, argv);
    g_object_unref(app);

    return status;
}
