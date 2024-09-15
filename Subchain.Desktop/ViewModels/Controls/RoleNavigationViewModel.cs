using Subchain.Tasks;
using Wpf.Ui.Controls;

namespace Subchain.Desktop.ViewModels.Controls
{
    public class RoleNavigationViewModel : ObservableObject
    {
        private Role _role;

        /// <summary>
        /// The name of the role.
        /// </summary>
        public string Name
        {
            get => _role.Name;
            set
            {
                _role.Name = value;
                OnPropertyChanged(nameof(Name));
            }
        }

        /// <summary>
        /// Creates a new RoleButtonViewModel.
        /// </summary>
        /// <param name="role">Role to initialise the button with.</param>
        public RoleNavigationViewModel(Role role)
        {
            _role = role;
        }

        public RoleNavigationViewModel()
        {
            _role = new Role("New role");
        }
    }
}
