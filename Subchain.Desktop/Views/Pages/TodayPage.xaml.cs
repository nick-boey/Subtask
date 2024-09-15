using Subchain.Desktop.ViewModels.Pages;
using Wpf.Ui.Controls;

namespace Subchain.Desktop.Views.Pages
{
    public partial class TodayPage : INavigableView<TodayViewModel>
    {
        public TodayViewModel ViewModel { get; }

        public TodayPage(TodayViewModel viewModel)
        {
            ViewModel = viewModel;
            DataContext = this;

            InitializeComponent();
        }
    }
}
