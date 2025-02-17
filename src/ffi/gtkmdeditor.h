#ifndef GTK_MD_EDITOR_H
#define GTK_MD_EDITOR_H

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define GTK_TYPE_MD_EDITOR (gtk_md_editor_get_type())
#define GTK_MD_EDITOR(obj) (G_TYPE_CHECK_INSTANCE_CAST((obj), GTK_TYPE_MD_EDITOR, GtkMdEditor))
#define GTK_IS_MD_EDITOR(obj) (G_TYPE_CHECK_INSTANCE_TYPE((obj), GTK_TYPE_MD_EDITOR))

typedef struct _GtkMdEditor GtkMdEditor;

GType gtk_md_editor_get_type(void);
GtkWidget* gtk_md_editor_new(void);

G_END_DECLS

#endif /* GTK_MD_EDITOR_H */
