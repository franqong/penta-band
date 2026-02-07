#include "MainWindow.h"
#include "Header.h"
#include <QVBoxLayout>
#include <QWidget>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    // Set window properties
    this->setWindowTitle("PentaBand");
    this->setMinimumSize(800, 600);

    // Main container widget and layout
    QWidget *centralWidget = new QWidget(this);
    QVBoxLayout *mainLayout = new QVBoxLayout(centralWidget);
    mainLayout->setContentsMargins(0, 0, 0, 0);
    mainLayout->setSpacing(0);

    // Add Header
    Header *header = new Header(this);
    mainLayout->addWidget(header);

    // Add Body
    QWidget *body = new QWidget(this);
    body->setStyleSheet("background-color: #191d24;");
    mainLayout->addWidget(body, 1); // The '1' makes the body stretch to fill available space

    // Set the central widget
    this->setCentralWidget(centralWidget);
}

MainWindow::~MainWindow()
{
}
