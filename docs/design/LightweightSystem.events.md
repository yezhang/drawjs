## Draw2D中监听的 SWT Canvas 事件

```java
protected void addListeners() {
  EventHandler handler = createEventHandler();
  canvas.getAccessible().addAccessibleListener(handler);
  canvas.getAccessible().addAccessibleControlListener(handler);
  canvas.addMouseListener(handler);
  canvas.addMouseMoveListener(handler);
  canvas.addMouseTrackListener(handler);
  canvas.addKeyListener(handler);
  canvas.addTraverseListener(handler);
  canvas.addFocusListener(handler);
  canvas.addDisposeListener(handler);
  canvas.addListener(SWT.MouseWheel, handler);

  canvas.addControlListener(new ControlAdapter() {
    public void controlResized(ControlEvent e) {
      LightweightSystem.this.controlResized();
    }
  });
  canvas.addListener(SWT.Paint, new Listener() {
    public void handleEvent(Event e) {
      LightweightSystem.this.paint(e.gc);
    }
  });
}
```

SWT 组件中的事件，请查看[这里](https://help.eclipse.org/2020-12/index.jsp?topic=/org.eclipse.platform.doc.isv/reference/api/org/eclipse/swt/accessibility/package-summary.html)

draw2d 所使用的事件有:
- **addAccessibleListener**: Adds the listener to the collection of listeners who will be notified when an accessible client asks for certain strings, such as name, description, help, or keyboard shortcut. The listener is notified by sending it one of the messages defined in the AccessibleListener interface.
- **addAccessibleControlListener**: Adds the listener to the collection of listeners who will be notified when an accessible client asks for custom control specific information.
- **addMouseListener**: MouseListener(*mouseDoubleClick*, *mouseDown*, *mouseUp*)
- **addMouseMoveListener**: MouseMoveListener(*mouseMove*)
    - Classes which implement this interface provide a method that deals with the events that are generated as the mouse pointer moves.
- **addMouseTrackListener**: MouseTrackListener(*mouseEnter*, *mouseExit*, *mouseHover*)
    - Classes which implement this interface provide methods that deal with the events that are generated as the mouse pointer passes (or hovers) over controls.
- **addKeyListener**: KeyListener(*keyPresed*, *keyReleased*)
    - Classes which implement this interface provide methods that deal with the events that are generated as keys are pressed on the system keyboard.
- **addTraverseListener**: TraverseListener(*keyTraversed*)
    - Classes which implement this interface provide a method that deals with the events that are generated when a traverse event occurs in a control.
    - A traverse event occurs when the user presses a traversal key. Traversal keys are typically **tab** and **arrow** keys, along with certain other keys on some platforms. Traversal key constants beginning with TRAVERSE_ are defined in the SWT class.
- **addFocusListener**: FocusListener(*focusGained*, *focusLost*)
    - Classes which implement this interface provide methods that deal with the events that are generated as controls gain and lose focus.
- **addDisposeListener**: DisposeListener(*widgetDisposed​*)
    - Classes which implement this interface provide a method that deals with the event that is generated when a widget is disposed.
- SWT.MouseWheel
- controlResized
- SWT.Paint

accessible 相关的时间，直接通过 `<canvas />` 标签的属性梳理即可，https://pauljadam.com/demos/canvas.html


