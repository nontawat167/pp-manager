import MainLayout from '@Components/layout/MainLayout';
import { MantineProvider } from '@mantine/core';
import { MemoryRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import AppConfigProvider, { useAppConfig } from './config/ConfigContext';
import Routes from './routes/AppRoutes';

const App = () => {
  const { initPath } = useAppConfig();
  const queryClient = new QueryClient();

  return (
    <QueryClientProvider client={queryClient}>
      <AppConfigProvider>
        <MantineProvider
          theme={{ fontFamily: 'Prompt, sans-serif' }}
          withGlobalStyles
          withNormalizeCSS
        >
          <MemoryRouter initialEntries={[initPath]}>
            <MainLayout>
              <Routes />
            </MainLayout>
          </MemoryRouter>
        </MantineProvider>
      </AppConfigProvider>
    </QueryClientProvider>
  );
};

export default App;
