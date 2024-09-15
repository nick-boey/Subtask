using Subchain.Desktop.ViewModels.Controls;
using System.Windows.Data;
using Wpf.Ui.Controls;

namespace Subchain.Desktop.Views.Controls
{
    /// <summary>
    /// Follow steps 1a or 1b and then 2 to use this custom control in a XAML file.
    ///
    /// Step 1a) Using this custom control in a XAML file that exists in the current project.
    /// Add this XmlNamespace attribute to the root element of the markup file where it is 
    /// to be used:
    ///
    ///     xmlns:MyNamespace="clr-namespace:Subchain.Desktop"
    ///
    ///
    /// Step 1b) Using this custom control in a XAML file that exists in a different project.
    /// Add this XmlNamespace attribute to the root element of the markup file where it is 
    /// to be used:
    ///
    ///     xmlns:MyNamespace="clr-namespace:Subchain.Desktop;assembly=Subchain.Desktop"
    ///
    /// You will also need to add a project reference from the project where the XAML file lives
    /// to this project and Rebuild to avoid compilation errors:
    ///
    ///     Right click on the target project in the Solution Explorer and
    ///     "Add Reference"->"Projects"->[Browse to and select this project]
    ///
    ///
    /// Step 2)
    /// Go ahead and use your control in the XAML file.
    ///
    ///     <MyNamespace:RoleNavigationViewItem/>
    ///
    /// </summary>
    public class RoleNavigationViewItem : NavigationViewItem
    {
        public RoleNavigationViewModel ViewModel { get; set; }
        public RoleNavigationViewItem(RoleNavigationViewModel viewModel)
        {
            ViewModel = viewModel;

            var binding = new Binding("ViewModel.Name")
            {
                Source = this,
                Mode = BindingMode.OneWay
            };

            this.SetBinding(ContentProperty, binding);
        }
    }
}
