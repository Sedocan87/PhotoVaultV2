import { render, screen } from '@testing-library/react';
import { Settings } from '../Settings';

test('renders settings page', () => {
    render(<Settings />);
    const linkElement = screen.getByText(/Settings/i);
    expect(linkElement).toBeInTheDocument();
});
