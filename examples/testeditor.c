#include <gtk/gtk.h>
#include "gtkmdeditor.h"

const char* example_text = "# Markdown Test Document\n\
\n\
## Formatting\n\
\n\
This is a **bold** text and this is *italic*.\n\
You can also use __bold__ and _italic_ with underscores.\n\
\n\
### Links\n\
\n\
Here's a [link to Google](https://google.com)\n\
And here's an image: ![cute cat](cat.jpg)\n\
\n\
#### Lists and More\n\
\n\
##### Small Heading\n\
\n\
This editor supports various markdown features!";

static void activate(GtkApplication *app, gpointer user_data) {
    GtkWidget *window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Markdown Editor Test");
    gtk_window_set_default_size(GTK_WINDOW(window), 600, 400);

    // Create a scrolled window
    GtkWidget *scrolled = gtk_scrolled_window_new();
    gtk_window_set_child(GTK_WINDOW(window), scrolled);

    // Add the editor to the scrolled window
    GtkWidget *editor = gtk_md_editor_new();
    GtkTextBuffer *text_buffer = gtk_text_view_get_buffer((GtkTextView*) editor);
    gtk_text_buffer_set_text(text_buffer, example_text, -1);
    gtk_scrolled_window_set_child(GTK_SCROLLED_WINDOW(scrolled), editor);

    gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char **argv) {
    GtkApplication *app = gtk_application_new("org.gtk.example", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
    int status = g_application_run(G_APPLICATION(app), argc, argv);
    g_object_unref(app);

    return status;
}
