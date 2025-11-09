#ifndef GTK_MD_EDITOR_H
#define GTK_MD_EDITOR_H

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define GTK_TYPE_MD_EDITOR (gtk_md_viewer_get_type())
#define GTK_MD_EDITOR(obj) (G_TYPE_CHECK_INSTANCE_CAST((obj), GTK_TYPE_MD_EDITOR, GtkMdViewer))
#define GTK_IS_MD_EDITOR(obj) (G_TYPE_CHECK_INSTANCE_TYPE((obj), GTK_TYPE_MD_EDITOR))

typedef struct _GtkMdViewer GtkMdViewer;

GType gtk_md_viewer_get_type(void);
GtkWidget* gtk_md_viewer_new(const char* md_text);
GtkWidget* gtk_md_viewer_new_with_img_prefix(const char* md_text, const char* img_prefix_ptr);

G_END_DECLS

#endif /* GTK_MD_EDITOR_H */
