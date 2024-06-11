/**
 * 表示 DOM 元素对应的鼠标字符串常量。
 * https://www.w3schools.com/jsref/prop_style_cursor.asp
 */
export enum CursorState {
  alias = 'alias', //The cursor indicates an alias of something is to be created
  all_scroll = 'all-scroll', //The cursor indicates that something can be scrolled in any direction
  auto = 'auto', //Default. The browser sets a cursor
  cell = 'cell', //The cursor indicates that a cell (or set of cells) may be selected
  context_menu = 'context-menu', //The cursor indicates that a context-menu is available
  col_resize = 'col-resize', //The cursor indicates that the column can be resized horizontally
  copy = 'copy', //The cursor indicates something is to be copied
  crosshair = 'crosshair', //The cursor render as a crosshair
  default = 'default', //The default cursor
  e_resize = 'e-resize', //The cursor indicates that an edge of a box is to be moved right (east)
  ew_resize = 'ew-resize', //Indicates a bidirectional resize cursor
  help = 'help', //The cursor indicates that help is available
  move = 'move', //The cursor indicates something is to be moved
  n_resize = 'n-resize', //The cursor indicates that an edge of a box is to be moved up (north)
  ne_resize = 'ne-resize', //The cursor indicates that an edge of a box is to be moved up and right (north/east)
  nesw_resize = 'nesw-resize', //Indicates a bidirectional resize cursor
  ns_resize = 'ns-resize', //Indicates a bidirectional resize cursor
  nw_resize = 'nw-resize', //The cursor indicates that an edge of a box is to be moved up and left (north/west)
  nwse_resize = 'nwse-resize', //Indicates a bidirectional resize cursor
  no_drop = 'no-drop', //The cursor indicates that the dragged item cannot be dropped here
  none = 'none', //No cursor is rendered for the element
  not_allowed = 'not-allowed', //The cursor indicates that the requested action will not be executed
  pointer = 'pointer', //The cursor is a pointer and indicates a link
  progress = 'progress', //The cursor indicates that the program is busy (in progress)
  row_resize = 'row-resize', //The cursor indicates that the row can be resized vertically
  s_resize = 's-resize', //The cursor indicates that an edge of a box is to be moved down (south)
  se_resize = 'se-resize', //The cursor indicates that an edge of a box is to be moved down and right (south/east)
  sw_resize = 'sw-resize', //The cursor indicates that an edge of a box is to be moved down and left (south/west)
  text = 'text', //The cursor indicates text that may be selected
  URL = 'URL', //A comma separated list of URLs to custom cursors. Note: Always specify a generic cursor at the end of the list, in case none of the URL-defined cursors can be used
  vertical_text = 'vertical-text', //The cursor indicates vertical-text that may be selected
  w_resize = 'w-resize', //The cursor indicates that an edge of a box is to be moved left (west)
  wait = 'wait', //The cursor indicates that the program is busy
  zoom_in = 'zoom-in', //The cursor indicates that something can be zoomed in
  zoom_out = 'zoom-out', //The cursor indicates that something can be zoomed out
  initial = 'initial', //Sets this property to its default value.
  inherit = 'inherit' //Inherits this property from its parent element.
}