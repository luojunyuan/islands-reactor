use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

mod gen_attach;
mod gen_bindings;
mod gen_set_prop;
mod helpers;
mod metadata;
mod schema;
mod toml_parser;

const ISLANDS_UI_XAML_VERSION: &str = "0.1.0-local";
const REACTOR_WIDGETS_TOML: &str = include_str!("reactor_widgets.toml");

const XAML_MINIMAL_FILTERS: &[&str] = &[
    "Windows.ApplicationModel.Resources.Core.ResourceManager::{get_Current}",
    "Windows.ApplicationModel.Resources.Core.IResourceManager::{LoadPriFiles}",
    "Windows.Storage.IStorageFile",
    "Windows.Storage.StorageFile::{GetFileFromPathAsync}",
    "Windows.System.DispatcherQueue::{GetForCurrentThread}",
    "Windows.System.DispatcherQueueHandler",
    "Windows.System.DispatcherQueuePriority",
    "Windows.System.IDispatcherQueue::{TryEnqueue, TryEnqueueWithPriority}",
    "Windows.UI.Color",
    "Windows.UI.Text.FontWeight",
    "Windows.UI.ViewManagement.UIColorType",
    "Windows.UI.ViewManagement.UISettings::{CreateInstance}",
    "Windows.UI.ViewManagement.IUISettings3::{GetColorValue, add_ColorValuesChanged, remove_ColorValuesChanged}",
    "Windows.UI.Xaml.Application::{CreateInstance, get_Current}",
    "Windows.UI.Xaml.IApplication::{get_Resources, put_Resources, add_UnhandledException}",
    "Windows.UI.Xaml.IApplicationOverrides",
    "Windows.UI.Xaml.Controls.AutoSuggestBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IAutoSuggestBox::{get_Text, put_Text, put_PlaceholderText, put_Header, add_TextChanged, add_SuggestionChosen}",
    "Windows.UI.Xaml.Controls.IAutoSuggestBox2::{add_QuerySubmitted}",
    "Windows.UI.Xaml.Controls.AutoSuggestBoxQuerySubmittedEventArgs",
    "Windows.UI.Xaml.Controls.IAutoSuggestBoxQuerySubmittedEventArgs::{get_QueryText}",
    "Windows.UI.Xaml.Controls.AutoSuggestBoxSuggestionChosenEventArgs",
    "Windows.UI.Xaml.Controls.IAutoSuggestBoxSuggestionChosenEventArgs::{get_SelectedItem}",
    "Windows.UI.Xaml.Controls.AutoSuggestBoxTextChangedEventArgs",
    "Windows.UI.Xaml.Controls.IAutoSuggestBoxTextChangedEventArgs::{get_Reason}",
    "Windows.UI.Xaml.Controls.AutoSuggestionBoxTextChangeReason",
    "Windows.UI.Xaml.Controls.Border::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IBorder::{put_Child, put_Padding, put_Background, put_CornerRadius, put_BorderBrush, put_BorderThickness}",
    "Windows.UI.Xaml.Controls.Button::{CreateInstance}",
    "Windows.UI.Xaml.Controls.CalendarDatePicker::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ICalendarDatePicker::{put_Header, put_PlaceholderText, put_IsCalendarOpen, put_IsTodayHighlighted}",
    "Windows.UI.Xaml.Controls.CalendarView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ICalendarView::{put_IsTodayHighlighted, put_IsGroupLabelVisible, add_SelectedDatesChanged}",
    "Windows.UI.Xaml.Controls.Canvas::{CreateInstance, SetLeft, SetTop, SetZIndex}",
    "Windows.UI.Xaml.Controls.CheckBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ColorChangedEventArgs",
    "Windows.UI.Xaml.Controls.ColorPicker::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IColorChangedEventArgs::{get_NewColor}",
    "Windows.UI.Xaml.Controls.IColorPicker::{put_Color, put_IsAlphaEnabled, put_IsHexInputVisible, put_IsColorSliderVisible, put_IsColorChannelTextInputVisible, add_ColorChanged}",
    "Windows.UI.Xaml.Controls.ColumnDefinition::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IColumnDefinition::{put_Width}",
    "Windows.UI.Xaml.Controls.ColumnDefinitionCollection",
    "Windows.UI.Xaml.Controls.ComboBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IComboBox2::{put_Header, put_PlaceholderText}",
    "Windows.UI.Xaml.Controls.IComboBox6::{put_IsEditable}",
    "Windows.UI.Xaml.Controls.CommandBar::{CreateInstance}",
    "Windows.UI.Xaml.Controls.CommandBarDefaultLabelPosition",
    "Windows.UI.Xaml.Controls.ICommandBar3::{put_DefaultLabelPosition}",
    "Windows.UI.Xaml.Controls.ContentControl",
    "Windows.UI.Xaml.Controls.IContentControl::{put_Content}",
    "Windows.UI.Xaml.Controls.ContentDialog::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IContentDialog::{put_Title, put_PrimaryButtonText, put_SecondaryButtonText, put_IsPrimaryButtonEnabled, put_IsSecondaryButtonEnabled}",
    "Windows.UI.Xaml.Controls.IContentDialog2::{put_CloseButtonText}",
    "Windows.UI.Xaml.Controls.Control",
    "Windows.UI.Xaml.Controls.IControl::{put_IsEnabled, put_Padding, put_Background, put_Foreground, put_FontSize, put_FontWeight, put_FontFamily}",
    "Windows.UI.Xaml.Controls.DatePicker::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IDatePicker::{put_Header, put_DayVisible, put_MonthVisible, put_YearVisible, add_DateChanged}",
    "Windows.UI.Xaml.Controls.IDatePickerValueChangedEventArgs::{get_NewDate}",
    "Windows.UI.Xaml.Controls.DropDownButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.FlipView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Grid::{CreateInstance, SetColumn, SetColumnSpan, SetRow, SetRowSpan}",
    "Windows.UI.Xaml.Controls.IGrid::{get_RowDefinitions, get_ColumnDefinitions}",
    "Windows.UI.Xaml.Controls.IGrid3::{put_RowSpacing, put_ColumnSpacing}",
    "Windows.UI.Xaml.Controls.GridView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.HyperlinkButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IconElement",
    "Windows.UI.Xaml.Controls.Image::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IImage::{put_Source, put_Stretch}",
    "Windows.UI.Xaml.Controls.ItemCollection",
    "Windows.UI.Xaml.Controls.ItemsControl",
    "Windows.UI.Xaml.Controls.IItemsControl::{get_Items, put_ItemsSource}",
    "Windows.UI.Xaml.Controls.ListBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ListView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ListViewSelectionMode",
    "Windows.UI.Xaml.Controls.IListViewBase::{put_SelectionMode}",
    "Windows.UI.Xaml.Controls.MenuBar::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Orientation",
    "Windows.UI.Xaml.Controls.Panel",
    "Windows.UI.Xaml.Controls.IPanel::{get_Children, put_Background}",
    "Windows.UI.Xaml.Controls.PasswordBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IPasswordBox::{get_Password, put_IsPasswordRevealButtonEnabled, add_PasswordChanged}",
    "Windows.UI.Xaml.Controls.IPasswordBox2::{put_Header, put_PlaceholderText}",
    "Windows.UI.Xaml.Controls.IPasswordBox3::{put_PasswordRevealMode}",
    "Windows.UI.Xaml.Controls.PasswordRevealMode",
    "Windows.UI.Xaml.Controls.PersonPicture::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IPersonPicture::{put_DisplayName, put_Initials}",
    "Windows.UI.Xaml.Controls.Pivot::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IPivot::{put_SelectedIndex, put_Title}",
    "Windows.UI.Xaml.Controls.PivotItem::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ProgressBar::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IProgressBar::{put_IsIndeterminate}",
    "Windows.UI.Xaml.Controls.ProgressRing::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IProgressRing::{put_IsActive}",
    "Windows.UI.Xaml.Controls.RadioButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRadioButton::{put_GroupName}",
    "Windows.UI.Xaml.Controls.RatingControl::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRatingControl::{get_Value, put_Value, put_Caption, put_IsReadOnly, put_MaxRating, put_PlaceholderValue, add_ValueChanged}",
    "Windows.UI.Xaml.Controls.RelativePanel::{CreateInstance, SetAlignBottomWithPanel, SetAlignHorizontalCenterWithPanel, SetAlignLeftWithPanel, SetAlignRightWithPanel, SetAlignTopWithPanel, SetAlignVerticalCenterWithPanel}",
    "Windows.UI.Xaml.Controls.RichEditBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRichEditBox::{put_IsReadOnly}",
    "Windows.UI.Xaml.Controls.IRichEditBox2::{put_Header, put_PlaceholderText}",
    "Windows.UI.Xaml.Controls.RichTextBlock::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRichTextBlock::{get_Blocks, put_FontSize, put_FontFamily, put_Foreground}",
    "Windows.UI.Xaml.Controls.RowDefinition::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IRowDefinition::{put_Height}",
    "Windows.UI.Xaml.Controls.RowDefinitionCollection",
    "Windows.UI.Xaml.Controls.ScrollBarVisibility",
    "Windows.UI.Xaml.Controls.ScrollViewer::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IScrollViewer::{put_HorizontalScrollBarVisibility, put_VerticalScrollBarVisibility}",
    "Windows.UI.Xaml.Controls.SelectionChangedEventArgs",
    "Windows.UI.Xaml.Controls.SelectionChangedEventHandler",
    "Windows.UI.Xaml.Controls.Slider::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ISlider::{put_Orientation, put_StepFrequency}",
    "Windows.UI.Xaml.Controls.ISlider2::{put_Header}",
    "Windows.UI.Xaml.Controls.SplitButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ISplitButton::{add_Click}",
    "Windows.UI.Xaml.Controls.SplitView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ISplitView::{put_IsPaneOpen, put_OpenPaneLength, put_CompactPaneLength, put_DisplayMode, put_Content, put_Pane, add_PaneClosed}",
    "Windows.UI.Xaml.Controls.SplitViewDisplayMode",
    "Windows.UI.Xaml.Controls.StackPanel::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IStackPanel::{put_Orientation}",
    "Windows.UI.Xaml.Controls.IStackPanel4::{put_Spacing}",
    "Windows.UI.Xaml.Controls.Symbol",
    "Windows.UI.Xaml.Controls.SymbolIcon::{CreateInstanceWithSymbol}",
    "Windows.UI.Xaml.Controls.TextBlock::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ITextBlock::{put_Text, put_TextWrapping, put_IsTextSelectionEnabled, put_FontSize, put_FontWeight, put_FontFamily, put_Foreground}",
    "Windows.UI.Xaml.Controls.TextBox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ITextBox::{get_Text, put_Text, put_TextWrapping, put_AcceptsReturn, add_TextChanged}",
    "Windows.UI.Xaml.Controls.ITextBox2::{put_Header, put_PlaceholderText}",
    "Windows.UI.Xaml.Controls.TimePicker::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ITimePicker::{put_Header, put_ClockIdentifier, put_MinuteIncrement, add_TimeChanged}",
    "Windows.UI.Xaml.Controls.ITimePickerValueChangedEventArgs::{get_NewTime}",
    "Windows.UI.Xaml.Controls.ToggleSwitch::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IToggleSwitch::{get_IsOn, put_IsOn, put_OnContent, put_OffContent, put_Header, add_Toggled}",
    "Windows.UI.Xaml.Controls.ToolTipService::{SetToolTip}",
    "Windows.UI.Xaml.Controls.TreeView::{CreateInstance}",
    "Windows.UI.Xaml.Controls.ITreeView::{put_SelectionMode, add_ItemInvoked}",
    "Windows.UI.Xaml.Controls.ITreeViewItemInvokedEventArgs::{get_InvokedItem}",
    "Windows.UI.Xaml.Controls.ITreeViewNode::{get_Content}",
    "Windows.UI.Xaml.Controls.TreeViewSelectionMode",
    "Windows.UI.Xaml.Controls.UIElementCollection",
    "Windows.UI.Xaml.Controls.Viewbox::{CreateInstance}",
    "Windows.UI.Xaml.Controls.IViewbox::{put_Child, put_Stretch}",
    "Windows.UI.Xaml.Controls.Primitives.ButtonBase",
    "Windows.UI.Xaml.Controls.Primitives.IButtonBase::{add_Click}",
    "Windows.UI.Xaml.Controls.Primitives.FlyoutPlacementMode",
    "Windows.UI.Xaml.Controls.Primitives.RangeBase",
    "Windows.UI.Xaml.Controls.Primitives.IRangeBase::{get_Value, put_Value, put_Minimum, put_Maximum, add_ValueChanged}",
    "Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs",
    "Windows.UI.Xaml.Controls.Primitives.IRangeBaseValueChangedEventArgs::{get_NewValue}",
    "Windows.UI.Xaml.Controls.Primitives.RepeatButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Primitives.IRepeatButton::{put_Delay, put_Interval}",
    "Windows.UI.Xaml.Controls.Primitives.Selector",
    "Windows.UI.Xaml.Controls.Primitives.ISelector::{get_SelectedIndex, put_SelectedIndex, add_SelectionChanged}",
    "Windows.UI.Xaml.Controls.Primitives.ToggleButton::{CreateInstance}",
    "Windows.UI.Xaml.Controls.Primitives.IToggleButton::{put_IsChecked, add_Checked, add_Unchecked}",
    "Windows.UI.Xaml.CornerRadius",
    "Windows.UI.Xaml.DataTemplate::{CreateInstance}",
    "Windows.UI.Xaml.DependencyObject",
    "Windows.UI.Xaml.DispatcherTimer::{CreateInstance}",
    "Windows.UI.Xaml.IDispatcherTimer::{put_Interval, add_Tick, Start, Stop}",
    "Windows.UI.Xaml.Documents.Block",
    "Windows.UI.Xaml.Documents.BlockCollection",
    "Windows.UI.Xaml.Documents.IParagraph::{get_Inlines}",
    "Windows.UI.Xaml.Documents.Inline",
    "Windows.UI.Xaml.Documents.InlineCollection",
    "Windows.UI.Xaml.Documents.IRun::{put_Text}",
    "Windows.UI.Xaml.Documents.Paragraph::{CreateInstance}",
    "Windows.UI.Xaml.Documents.Run::{CreateInstance}",
    "Windows.UI.Xaml.ElementTheme",
    "Windows.UI.Xaml.FrameworkElement",
    "Windows.UI.Xaml.IFrameworkElement::{put_VerticalAlignment, put_HorizontalAlignment, put_Margin, put_Height, put_Width, put_MinWidth, put_MaxWidth, put_MinHeight, put_MaxHeight, put_Tag, get_Tag, put_Style}",
    "Windows.UI.Xaml.IFrameworkElement2::{put_RequestedTheme}",
    "Windows.UI.Xaml.IFrameworkElement6::{get_ActualTheme, add_ActualThemeChanged}",
    "Windows.UI.Xaml.GridLength",
    "Windows.UI.Xaml.GridUnitType",
    "Windows.UI.Xaml.HorizontalAlignment",
    "Windows.UI.Xaml.Hosting.DesktopWindowXamlSource::{CreateInstance}",
    "Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource::{put_Content}",
    "Windows.UI.Xaml.Hosting.WindowsXamlManager::{InitializeForCurrentThread}",
    "Windows.UI.Xaml.Interop.TypeName",
    "Windows.UI.Xaml.Markup.IXamlMetadataProvider",
    "Windows.UI.Xaml.Markup.IXamlType",
    "Windows.UI.Xaml.Markup.XamlReader::{Load}",
    "Windows.UI.Xaml.Markup.XmlnsDefinition",
    "Windows.UI.Xaml.Media.Brush",
    "Windows.UI.Xaml.Media.CompositionTarget::{add_Rendering}",
    "Windows.UI.Xaml.Media.FontFamily::{CreateInstanceWithName}",
    "Windows.UI.Xaml.Media.ImageSource",
    "Windows.UI.Xaml.Media.Imaging.BitmapImage::{CreateInstance}",
    "Windows.UI.Xaml.Media.Imaging.IBitmapImage::{put_UriSource}",
    "Windows.UI.Xaml.Media.SolidColorBrush::{CreateInstance}",
    "Windows.UI.Xaml.Media.ISolidColorBrush::{put_Color}",
    "Windows.UI.Xaml.Media.Stretch",
    "Windows.UI.Xaml.ResourceDictionary::{CreateInstance}",
    "Windows.UI.Xaml.IResourceDictionary::{get_MergedDictionaries}",
    "Windows.UI.Xaml.RoutedEventArgs",
    "Windows.UI.Xaml.RoutedEventHandler",
    "Windows.UI.Xaml.Shapes.Ellipse::{CreateInstance}",
    "Windows.UI.Xaml.Shapes.Line::{CreateInstance}",
    "Windows.UI.Xaml.Shapes.ILine::{put_X1, put_Y1, put_X2, put_Y2}",
    "Windows.UI.Xaml.Shapes.Rectangle::{CreateInstance}",
    "Windows.UI.Xaml.Shapes.IRectangle::{put_RadiusX, put_RadiusY}",
    "Windows.UI.Xaml.Shapes.Shape",
    "Windows.UI.Xaml.Shapes.IShape::{put_Fill, put_Stroke, put_StrokeThickness}",
    "Windows.UI.Xaml.Style::{CreateInstance}",
    "Windows.UI.Xaml.TextWrapping",
    "Windows.UI.Xaml.Thickness",
    "Windows.UI.Xaml.UIElement",
    "Windows.UI.Xaml.IUIElement::{put_Opacity}",
    "Windows.UI.Xaml.UnhandledExceptionEventArgs",
    "Windows.UI.Xaml.IUnhandledExceptionEventArgs::{get_Exception, get_Message}",
    "Windows.UI.Xaml.UnhandledExceptionEventHandler",
    "Windows.UI.Xaml.VerticalAlignment",
    "Windows.Win32.Foundation.HWND",
    "Windows.Win32.System.WinRT.Xaml.IDesktopWindowXamlSourceNative::{AttachToWindow, get_WindowHandle}",
];

const IUXC_MINIMAL_FILTERS: &[&str] = &[
    "Islands.UI.Xaml.Controls.ScrollView::{CreateInstance}",
    "Islands.UI.Xaml.Controls.IScrollView::{put_Content, put_HorizontalScrollBarVisibility, put_VerticalScrollBarVisibility}",
    "Islands.UI.Xaml.Controls.ScrollingScrollBarVisibility",
    "Islands.UI.Xaml.Controls.TitleBar::{CreateInstance}",
    "Islands.UI.Xaml.Controls.ITitleBar::{put_Title, put_Subtitle, put_Content, put_RightHeader, put_IsBackButtonVisible, put_IsBackButtonEnabled, put_IsPaneToggleButtonVisible, add_BackRequested, add_PaneToggleRequested}",
    "Islands.UI.Xaml.Controls.ITitleBar2::{put_AutoRefreshDragRegions, RecomputeDragRegions}",
    "Islands.UI.Xaml.Controls.AppWindowTitleBar::{CreateInstance}",
    "Islands.UI.Xaml.Controls.IAppWindowTitleBar::{put_ExtendsContentIntoTitleBar, put_LeftInset, put_RightInset}",
    "Islands.UI.Xaml.Controls.TitleBarWindowAdapter::{CreateInstance}",
    "Islands.UI.Xaml.Controls.ITitleBarWindowAdapter::{put_WindowHandle, get_WindowTitleBar, SetTitleBar, NotifyWindowActivated, SetCaptionInsets, HitTest, ApplyTitleBarWindowRegion}",
    "Islands.UI.Xaml.Controls.XamlMetaDataProvider::{CreateInstance}",
];

const BACKEND_GENERATED_MODULE: &str = "mod attach_event;\n\
mod set_prop;\n\
pub(super) use attach_event::dispatch as dispatch_attach_event;\n\
pub(super) use set_prop::dispatch as dispatch_set_prop;\n";

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("generate-bindings") if args.next().is_none() => {
            if let Err(err) = generate_bindings() {
                eprintln!("islands-reactor-codegen: {err}");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage: cargo run -p islands-reactor-codegen -- generate-bindings");
            std::process::exit(2);
        }
    }
}

fn generate_bindings() -> Result<(), String> {
    let root = workspace_root()?;
    let iuxc_package = islands_ui_xaml_package_dir(&root)?;
    let iuxc_winmd = iuxc_native_asset(&iuxc_package, "x64", "Islands.UI.Xaml.Controls.winmd")?;

    generate_xaml_bindings(&root)?;
    generate_iuxc_module(&root, &iuxc_winmd)?;
    generate_reactor_code(&root, &iuxc_winmd)?;
    Ok(())
}

fn generate_xaml_bindings(root: &Path) -> Result<(), String> {
    let out = root.join("crates/islands-reactor/src/native/bindings.rs");
    let temp = root
        .join("target")
        .join("islands-reactor-codegen")
        .join("bindings.rs");
    let temp_arg = path_arg(&temp);
    let mut args = vec![
        "--in",
        "default",
        "--out",
        &temp_arg,
        "--implement",
        "Windows.UI.Xaml.IApplicationOverrides",
        "Windows.UI.Xaml.Markup.IXamlMetadataProvider",
        "--minimal",
        "--flat",
        "--filter",
    ];
    args.extend(XAML_MINIMAL_FILTERS);
    args.extend([
        "--reference",
        "windows,skip-root,Windows.ApplicationModel.Core",
        "--reference",
        "windows,skip-root,Windows.Foundation.Collections.IPropertySet",
        "--reference",
        "windows,skip-root,Windows.Foundation.Collections.PropertySet",
        "--reference",
        "windows,skip-root,Windows.Foundation.Collections.ValueSet",
        "--reference",
        "windows_collections,flat,Windows.Foundation.Collections",
        "--reference",
        "windows,skip-root,Windows.Foundation.Numerics.Quaternion",
        "--reference",
        "windows_numerics,flat,Windows.Foundation.Numerics",
        "--reference",
        "windows,skip-root,Windows.Foundation",
        "--reference",
        "windows,skip-root,Windows.Win32.Foundation",
        "--reference",
        "windows,skip-root,Windows.Win32.UI.WindowsAndMessaging",
    ]);
    run_bindgen(args)?;
    patch_xaml_bindings(&temp)?;
    copy_if_changed(&temp, &out)
}

fn generate_iuxc_module(root: &Path, iuxc_winmd: &Path) -> Result<(), String> {
    let out = root.join("crates/islands-reactor/src/native/bindings_iuxc.rs");
    let temp = root
        .join("target")
        .join("islands-reactor-codegen")
        .join("bindings_iuxc.rs");
    let temp_arg = path_arg(&temp);
    let iuxc_winmd = path_arg(iuxc_winmd);
    let mut args = vec![
        "--in",
        "default",
        &iuxc_winmd,
        "--out",
        &temp_arg,
        "--minimal",
        "--flat",
        "--filter",
    ];
    args.extend(IUXC_MINIMAL_FILTERS);
    args.extend([
        "--reference",
        "crate::bindings,flat,Windows.UI",
        "--reference",
        "crate::bindings,flat,Windows.UI.Xaml",
        "--reference",
        "crate::bindings,flat,Windows.UI.Xaml.Controls",
        "--reference",
        "crate::bindings,flat,Windows.UI.Input",
        "--reference",
        "crate::bindings,flat,Windows.UI.Text",
        "--reference",
        "windows,skip-root,Windows.Foundation",
        "--reference",
        "windows_collections,flat,Windows.Foundation.Collections",
        "--reference",
        "windows,skip-root,Windows.Win32.Foundation",
        "--reference",
        "windows,skip-root,Windows.Foundation.Numerics.Quaternion",
        "--reference",
        "windows_numerics,flat,Windows.Foundation.Numerics",
    ]);
    run_bindgen(args)?;
    patch_iuxc_bindings(&temp)?;
    copy_if_changed(&temp, &out)
}

fn generate_reactor_code(root: &Path, iuxc_winmd: &Path) -> Result<(), String> {
    let metadata_dir = stage_reactor_metadata(root, iuxc_winmd)?;
    let resolver = metadata::MetadataResolver::load(&metadata_dir);
    let mut controls = toml_parser::parse(REACTOR_WIDGETS_TOML, &resolver);

    for ctrl in &mut controls {
        let handle = ctrl.handle().to_string();
        for prop in &mut ctrl.prop {
            prop.resolve_defaults(Some(&resolver), &handle);
        }
    }

    for warning in schema::validate(&controls, &resolver) {
        eprintln!("{warning}");
    }

    write_generated_rs(
        &root.join("crates/islands-reactor/src/generated.rs"),
        &gen_bindings::generate(&controls),
    )?;
    write_generated_rs(
        &root.join("crates/islands-reactor/src/native/winui/backend/generated/set_prop.rs"),
        &gen_set_prop::generate(&controls, &resolver),
    )?;
    write_generated_rs(
        &root.join("crates/islands-reactor/src/native/winui/backend/generated/attach_event.rs"),
        &gen_attach::generate(&controls, &resolver),
    )?;
    write_generated_rs(
        &root.join("crates/islands-reactor/src/native/winui/backend/generated.rs"),
        BACKEND_GENERATED_MODULE,
    )
}

fn stage_reactor_metadata(root: &Path, iuxc_winmd: &Path) -> Result<PathBuf, String> {
    let work = root
        .join("target")
        .join("islands-reactor-codegen")
        .join("reactor-metadata");
    recreate_dir(&work)?;

    let windows_winmd = locate_windows_bindgen_default_winmd(root)?;
    copy_file(&windows_winmd, &work.join("Windows.winmd"))?;
    copy_file(iuxc_winmd, &work.join("Islands.UI.Xaml.Controls.winmd"))?;
    Ok(work)
}

fn locate_windows_bindgen_default_winmd(root: &Path) -> Result<PathBuf, String> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(root.join("Cargo.toml"))
        .exec()
        .map_err(|err| format!("cargo metadata failed: {err}"))?;

    let package = metadata
        .packages
        .iter()
        .find(|package| package.name.as_str() == "windows-bindgen")
        .ok_or_else(|| "cargo metadata did not include windows-bindgen".to_string())?;
    let manifest = package.manifest_path.as_std_path();
    let winmd = manifest
        .parent()
        .ok_or_else(|| {
            format!(
                "windows-bindgen manifest has no parent: {}",
                manifest.display()
            )
        })?
        .join("default")
        .join("Windows.winmd");
    require_file(&winmd, "windows-bindgen default Windows.winmd")?;
    Ok(winmd)
}

fn write_generated_rs(path: &Path, code: &str) -> Result<(), String> {
    write_if_changed(path, &rustfmt(code))
}

fn run_bindgen(args: Vec<&str>) -> Result<(), String> {
    let _warnings = windows_bindgen::bindgen(args);
    Ok(())
}

fn write_if_changed(path: &Path, content: &str) -> Result<(), String> {
    if let Ok(existing) = fs::read_to_string(path)
        && existing == content
    {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(path, content).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn rustfmt(code: &str) -> String {
    let Ok(mut child) = Command::new("rustfmt")
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        return code.to_string();
    };
    if let Some(stdin) = child.stdin.as_mut() {
        let _ = stdin.write_all(code.as_bytes());
    }
    match child.wait_with_output() {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).unwrap_or_else(|_| code.to_string())
        }
        _ => code.to_string(),
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("could not resolve workspace root from {}", dir.display()))
}

fn islands_ui_xaml_package_dir(root: &Path) -> Result<PathBuf, String> {
    if let Ok(dir) = env::var("ISLANDS_UI_XAML_PACKAGE_DIR") {
        let dir = PathBuf::from(dir);
        if dir.exists() {
            return Ok(dir);
        }
        return Err(format!(
            "ISLANDS_UI_XAML_PACKAGE_DIR points to a missing directory: {}",
            dir.display()
        ));
    }

    if let Ok(package) = env::var("ISLANDS_UI_XAML_NUPKG") {
        let package = PathBuf::from(package);
        require_file(&package, "Islands.UI.Xaml NuGet package")?;
        return extract_iuxc_nupkg(root, &package);
    }

    if let Some(parent) = root.parent() {
        let package = parent
            .join("ixx-trans")
            .join("artifacts")
            .join("packages")
            .join(format!("Islands.UI.Xaml.{ISLANDS_UI_XAML_VERSION}.nupkg"));
        if package.exists() {
            return extract_iuxc_nupkg(root, &package);
        }
    }

    let nuget_root = if let Ok(root) = env::var("NUGET_PACKAGES") {
        PathBuf::from(root)
    } else {
        let profile = env::var("USERPROFILE").map_err(|_| "USERPROFILE is not set".to_string())?;
        PathBuf::from(profile).join(".nuget").join("packages")
    };
    let package_dir = nuget_root
        .join("islands.ui.xaml")
        .join(ISLANDS_UI_XAML_VERSION);
    if package_dir.exists() {
        Ok(package_dir)
    } else {
        Err(format!(
            "missing NuGet package Islands.UI.Xaml/{ISLANDS_UI_XAML_VERSION}; \
             set ISLANDS_UI_XAML_NUPKG or ISLANDS_UI_XAML_PACKAGE_DIR"
        ))
    }
}

fn extract_iuxc_nupkg(root: &Path, package: &Path) -> Result<PathBuf, String> {
    let work = root
        .join("target")
        .join("islands-reactor-codegen")
        .join("islands-ui-xaml-package");
    recreate_dir(&work)?;
    expand_nupkg(package, &work)?;
    Ok(work)
}

fn iuxc_native_asset(package: &Path, arch: &str, file: &str) -> Result<PathBuf, String> {
    let path = package
        .join("runtimes")
        .join(iuxc_rid(arch)?)
        .join("native")
        .join(file);
    require_file(&path, &format!("Islands.UI.Xaml {arch} native asset"))?;
    Ok(path)
}

fn iuxc_rid(arch: &str) -> Result<&'static str, String> {
    match arch {
        "x64" => Ok("win-x64"),
        "arm64" => Ok("win-arm64"),
        "x86" => Ok("win-x86"),
        _ => Err(format!("unsupported Islands.UI.Xaml architecture: {arch}")),
    }
}

fn expand_nupkg(nupkg: &Path, stage: &Path) -> Result<(), String> {
    run_command(
        Command::new("powershell")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(format!(
                "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force",
                ps_escape(nupkg),
                ps_escape(stage)
            )),
        "Expand-Archive",
    )
}

fn recreate_dir(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_dir_all(path)
            .map_err(|err| format!("failed to remove {}: {err}", path.display()))?;
    }
    fs::create_dir_all(path).map_err(|err| format!("failed to create {}: {err}", path.display()))
}

fn copy_file(src: &Path, dst: &Path) -> Result<(), String> {
    require_file(src, "source file")?;
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::copy(src, dst).map(|_| ()).map_err(|err| {
        format!(
            "failed to copy {} to {}: {err}",
            src.display(),
            dst.display()
        )
    })
}

fn copy_if_changed(src: &Path, dst: &Path) -> Result<(), String> {
    require_file(src, "generated bindings")?;
    if let (Ok(existing), Ok(generated)) = (fs::read(dst), fs::read(src))
        && existing == generated
    {
        return Ok(());
    }
    copy_file(src, dst)
}

fn patch_xaml_bindings(path: &Path) -> Result<(), String> {
    let mut code = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    patch_windows_core_api(&mut code);
    insert_selection_changed_event_handler_ctor(&mut code)?;
    fs::write(path, code).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn patch_iuxc_bindings(path: &Path) -> Result<(), String> {
    let mut code = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    code = code.replace("xaml_bindings::", "crate::bindings::");
    patch_windows_core_api(&mut code);
    fs::write(path, code).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn patch_windows_core_api(code: &mut String) {
    for name in ["Type", "CopyType", "CloneType"] {
        *code = code.replace(
            &format!("windows_core::{name}"),
            &format!("windows_core::imp::{name}"),
        );
    }
    *code = code
        .replace("windows_core::imp::Box::new", "std::boxed::Box::new")
        .replace("windows_core::Box::new", "std::boxed::Box::new");
}

fn insert_selection_changed_event_handler_ctor(code: &mut String) -> Result<(), String> {
    if code.contains("impl SelectionChangedEventHandler {\n    pub fn new<") {
        return Ok(());
    }

    let ctor = r#"struct SelectionChangedEventHandlerResultBox<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<SelectionChangedEventArgs>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> SelectionChangedEventHandlerResultBox<F>
{
    const VTABLE: SelectionChangedEventHandler_Vtbl = SelectionChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<SelectionChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        e: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<SelectionChangedEventHandler, F>);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&e),
            )
            .into()
        }
    }
}
impl SelectionChangedEventHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<SelectionChangedEventArgs>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    {
        let com = windows_core::imp::DelegateBox::<SelectionChangedEventHandler, _>::new(
            &SelectionChangedEventHandlerResultBox::<F>::VTABLE,
            handler,
        );
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
}
"#;

    for needle in [
        "#[repr(transparent)]\r\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\r\npub struct SelectionMode(pub i32);",
        "#[repr(transparent)]\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\npub struct SelectionMode(pub i32);",
        "#[repr(transparent)]\r\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\r\npub struct ListViewSelectionMode(pub i32);",
        "#[repr(transparent)]\n#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]\npub struct ListViewSelectionMode(pub i32);",
    ] {
        if let Some(index) = code.find(needle) {
            code.insert_str(index, ctor);
            return Ok(());
        }
    }

    Err("failed to patch SelectionChangedEventHandler constructor".to_string())
}

fn require_file(path: &Path, label: &str) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("{label} not found: {}", path.display()))
    }
}

fn run_command(command: &mut Command, label: &str) -> Result<(), String> {
    let output = command
        .output()
        .map_err(|err| format!("{label} failed to start: {err}"))?;
    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "{label} exited with {}{}\n{}",
            output.status,
            if stdout.trim().is_empty() {
                String::new()
            } else {
                format!("\nstdout:\n{stdout}")
            },
            stderr
        ))
    }
}

fn path_arg(path: &Path) -> String {
    path.display().to_string()
}

fn ps_escape(path: &Path) -> String {
    path.display().to_string().replace('\'', "''")
}
