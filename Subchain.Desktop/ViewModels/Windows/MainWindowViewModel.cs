using System.Collections.ObjectModel;
using Wpf.Ui.Controls;
using Subchain.Tasks;
using Subchain.Desktop.ViewModels.Controls;
using Subchain.Desktop.Views.Controls;

namespace Subchain.Desktop.ViewModels.Windows
{
    public partial class MainWindowViewModel : ObservableObject
    {
        [ObservableProperty]
        private string _applicationTitle = "Subchain";

        [ObservableProperty]
        private ObservableCollection<object> _menuItems = new()
        {
            new NavigationViewItem()
            {
                Content = "Today",
                Icon = new SymbolIcon { Symbol = SymbolRegular.Home24 },
                TargetPageType = typeof(Views.Pages.TodayPage)
            }
        };
        
        partial void OnMenuItemsChanged(ObservableCollection<object> value)
        {
            Console.WriteLine("Menu items changed");
        }

        [ObservableProperty]
        private ObservableCollection<object> _footerMenuItems = new()
        {
            new NavigationViewItem()
            {
                Content = "Settings",
                Icon = new SymbolIcon { Symbol = SymbolRegular.Settings24 },
                TargetPageType = typeof(Views.Pages.SettingsPage)
            }
        };

        [ObservableProperty]
        private ObservableCollection<MenuItem> _trayMenuItems = new()
        {
            new MenuItem { Header = "Home", Tag = "tray_home" }
        };

        [RelayCommand]
        private void AddRole()
        {
            Role role = new Role("New Role");
            var roleViewModel = new RoleNavigationViewModel(role);
            //var roleButton = new RoleNavigationViewItem(roleViewModel);
            var roleButton = new NavigationViewItem()
            {
                Content = "Test",
                Icon = new SymbolIcon { Symbol = SymbolRegular.Add24 },
                TargetPageType = typeof(Views.Pages.TodayPage)
            };
            MenuItems.Add(roleButton);
        }

        public MainWindowViewModel()
        {
            var addRoleButton = new NavigationViewItem()
            {
                Content = "Add Role",
                Icon = new SymbolIcon { Symbol = SymbolRegular.Add24 },
            };
            addRoleButton.Command = AddRoleCommand;

            FooterMenuItems.Insert(0, addRoleButton);
        }
    }
}
