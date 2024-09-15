using Subchain.Desktop.ViewModels.Pages;
using Wpf.Ui.Controls;

namespace Subchain.Desktop.Views.Pages
{
    public partial class RolePage : INavigableView<RoleViewModel>
    {
        public RoleViewModel ViewModel { get; }

        public RolePage(RoleViewModel viewModel)
        {
            ViewModel = viewModel;
            DataContext = this;

            InitializeComponent();
        }
    }
}
